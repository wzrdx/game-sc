#![no_std]

use core::iter::FromIterator;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/* For regular quests, the requirements/rewards slice contains the amount of tokens
in the following order: [Energy, Herbs, Gems, Essence].
For the final quest (mission), the sum of the elements in the rewards slice is equal
to the number of rewarded tickets. E.g. [1, 0, 0, 0] = 1 ticket
*/

// #[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
// pub struct Quest<M: ManagedTypeApi> {
//     pub id: u8,
//     pub duration: usize,
//     pub is_final: bool,
//     pub requirements: ManagedVec<M, u64>,
//     pub rewards: ManagedVec<M, u64>,
// }

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct Quest {
    pub id: u8,
    pub duration: usize,
    pub is_final: bool,
    pub requirements: [u64; 2],
    pub rewards: [u64; 2],
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct OngoingQuest {
    pub id: u8,
    pub end_timestamp: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct StakingInfo<M: ManagedTypeApi> {
    pub rewards: BigUint<M>,
    pub timestamp: u64,
    pub nonces: ManagedVec<M, u64>,
}

#[multiversx_sc::contract]
pub trait GameScContract: multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule {
    #[init]
    fn init(&self) {
        // self.quests().clear();

        // let quests = [
        //     Quest {
        //         id: 1,
        //         duration: 30,
        //         is_final: false,
        //         requirements: [1000000, 0],
        //         rewards: [0, 2000000],
        //     },
        //     Quest {
        //         id: 2,
        //         duration: 30,
        //         is_final: false,
        //         requirements: [0, 2000000],
        //         rewards: [500000, 0],
        //     },
        //     Quest {
        //         id: 3,
        //         duration: 60,
        //         is_final: true,
        //         requirements: [2000000, 2000000],
        //         rewards: [1, 0],
        //     },
        // ];

        // self.quests().extend_from_slice(&quests);
    }

    // #[only_owner]
    // #[endpoint(setQuests)]
    // fn set_quests(&self, quests: ManagedVec<Quest<Self::Api>>) {}

    #[only_owner]
    #[endpoint(setTokenId)]
    fn set_token_id(&self, token_id: TokenIdentifier) {
        self.nft_mapper().set_token_id(token_id);
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueTicketsCollection)]
    fn issue_tickets_collection(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();

        self.tickets_mapper().issue_and_set_all_roles(
            EsdtTokenType::SemiFungible,
            issue_cost,
            token_display_name,
            token_ticker,
            0 as usize,
            None,
        );
    }

    #[only_owner]
    #[endpoint(createTicketsToken)]
    fn create_tickets_token(&self, royalties: BigUint) {
        let attributes = self.build_attributes_buffer();
        let hash_buffer = self.crypto().sha256(&attributes);
        let attributes_hash = hash_buffer.as_managed_buffer();
        let uris = self.build_uris_vec();

        self.send().esdt_nft_create(
            &self.tickets_mapper().get_token_id(),
            &BigUint::from(1 as u32),
            &ManagedBuffer::new_from_bytes("Golden Ticket".as_bytes()),
            &royalties,
            &attributes_hash,
            &attributes,
            &uris,
        );
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueEnergyToken)]
    fn issue_energy_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();

        self.energy_mapper()
            .issue_and_set_all_roles(issue_cost, token_display_name, token_ticker, 6 as usize, None);
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueHerbsToken)]
    fn issue_herbs_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();

        self.herbs_mapper()
            .issue_and_set_all_roles(issue_cost, token_display_name, token_ticker, 6 as usize, None);
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers();
        self.nft_mapper().require_all_same_token(&payments);

        require!(payments.len() > 0, "Must stake at least one NFT");

        let caller = self.blockchain().get_caller();
        self.claim_staking_rewards_for_user(&caller);

        for payment in payments.into_iter() {
            self.staked_nonces(&caller).insert(payment.token_nonce);
        }
    }

    #[endpoint(unstake)]
    fn unstake(&self) {
        let caller = self.blockchain().get_caller();
        let token_id = self.nft_mapper().get_token_id();

        require!(
            self.staked_nonces(&caller).len() > 0,
            "Must have at least one staked NFT in order to unstake"
        );

        self.claim_staking_rewards_for_user(&caller);

        let mut payments: ManagedVec<EsdtTokenPayment> = ManagedVec::new();

        for nonce in self.staked_nonces(&caller).iter() {
            payments.push(EsdtTokenPayment::new(token_id.clone(), nonce, BigUint::from(1 as u32)))
        }

        if payments.len() > 0 {
            self.send().direct_multi(&caller, &payments);
            self.staked_nonces(&caller).clear();
            self.last_staking_timestamp(&caller).clear();
        }
    }

    #[endpoint(claimStakingRewards)]
    fn claim_staking_rewards(&self) {
        let caller = self.blockchain().get_caller();

        require!(
            self.staked_nonces(&caller).len() > 0,
            "Must have at least one staked NFT in order to unstake"
        );

        self.claim_staking_rewards_for_user(&caller);
    }

    #[payable("*")]
    #[endpoint(startQuest)]
    fn start_quest(&self, id: u8) {
        let caller = self.blockchain().get_caller();

        for ongoing_quest in self.ongoing_quests(&caller).iter() {
            require!(id != ongoing_quest.id, "Cannot start an already ongoing quest");
        }

        let quest = self.quests().get(id as usize);

        // Payment check & burning of tokens
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers();
        let requirements: [u64; 2] = quest.requirements;

        require!(
            payments.len() == requirements.iter().filter(|&x| *x > 0).count(),
            "Received incorrect number of payments"
        );

        let mut payment_index: usize = 0;

        for (i, requirement) in requirements.iter().enumerate() {
            if *requirement > 0 {
                let payment = payments.get(payment_index);
                let mapper = self.get_token_mapper(i);

                mapper.require_same_token(&payment.token_identifier);
                require!(payment.amount == BigUint::from(*requirement), "Incorrect payment");
                mapper.burn(&payment.amount);

                payment_index += 1;
            }
        }

        // Add to ongoing quests
        let current_timestamp = self.blockchain().get_block_timestamp();

        self.ongoing_quests(&caller).push(&OngoingQuest {
            id,
            end_timestamp: current_timestamp + (quest.duration as u64),
        });
    }

    #[endpoint(completeQuest)]
    fn complete_quest(&self, id: u8) {
        let caller = self.blockchain().get_caller();

        let mut search_result: Option<OngoingQuest> = None;
        let mut index_to_remove: usize = 0;

        for (i, q) in self.ongoing_quests(&caller).iter().enumerate() {
            if q.id == id {
                search_result = Some(q);
                index_to_remove = i + 1;
            }
        }

        let ongoing_quest = match search_result {
            Some(q) => q,
            None => sc_panic!("Ongoing quest not found"),
        };

        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(
            current_timestamp >= ongoing_quest.end_timestamp,
            "Quest cannot be completed yet"
        );

        // Rewards
        let quest = self.quests().get(id as usize);
        let rewards: [u64; 2] = quest.rewards;

        if quest.is_final {
            let tickets_amount: u64 = rewards.iter().sum();
            self.tickets_mapper()
                .nft_add_quantity_and_send(&caller, 1 as u64, BigUint::from(tickets_amount));
        } else {
            for (i, reward) in rewards.iter().enumerate() {
                if *reward > 0 {
                    let mapper = self.get_token_mapper(i);
                    mapper.mint_and_send(&caller, BigUint::from(*reward));
                }
            }
        }

        self.ongoing_quests(&caller).swap_remove(index_to_remove);
    }

    #[endpoint(faucet)]
    fn faucet(&self) {
        let caller = self.blockchain().get_caller();
        self.energy_mapper().mint_and_send(&caller, BigUint::from(10000000 as u32));
        self.herbs_mapper().mint_and_send(&caller, BigUint::from(10000000 as u32));
        self.tickets_mapper()
            .nft_add_quantity_and_send(&caller, 1 as u64, BigUint::from(5 as u32));
    }

    #[payable("*")]
    #[endpoint(joinRaffle)]
    fn join_raffle(&self) {
        let payment: EsdtTokenPayment = self.call_value().single_esdt();
        self.tickets_mapper().require_same_token(&payment.token_identifier);

        let caller = self.blockchain().get_caller();

        if self.raffle_participant_id(&caller).is_empty() {
            let id: u16 = self.raffle_index().update(|i| {
                *i += 1;
                *i
            });

            self.raffle_participant_id(&caller).set(id);
            self.raffle_id_participant(id).set(&caller);
        }

        let participant_id = self.raffle_participant_id(&caller).get();

        for _ in 0..payment.amount.to_u64().unwrap_or_default() {
            self.raffle_vector().push(&participant_id);
        }

        self.tickets_mapper().nft_burn(1 as u64, &payment.amount);
    }

    #[only_owner]
    #[endpoint(drawRaffleWinner)]
    fn draw_raffle_winner(&self) {
        let mut rand_source = RandomnessSource::new();
        let index: usize = rand_source.next_usize_in_range(1, self.raffle_vector().len() + 1);

        let winner_id: u16 = self.raffle_vector().get(index);
        let winner_address = self.raffle_id_participant(winner_id).get();

        self.send()
            .direct_egld(&winner_address, &BigUint::from(50000000000000000 as u64));

        // TODO: Store current raffle participants count
        // TODO: Set number of winners as min(20, participants)

        let mut vector: ManagedVec<u16>;
        vector = ManagedVec::from_iter(self.raffle_vector().iter().filter(|id| *id != winner_id));

        for id in vector.iter() {
            self.test_vector().push(&id);
        }
    }

    #[only_owner]
    #[endpoint(clearOngoingQuests)]
    fn clear_ongoing_quests(&self, user: &ManagedAddress) {
        self.ongoing_quests(user).clear();
    }

    #[view(getStakingInfo)]
    fn get_staking_info(&self, user: &ManagedAddress) -> StakingInfo<Self::Api> {
        let mut nonces: ManagedVec<u64> = ManagedVec::new();

        for nonce in self.staked_nonces(user).iter() {
            nonces.push(nonce)
        }

        StakingInfo {
            rewards: self.get_staking_rewards(user),
            timestamp: self.last_staking_timestamp(user).get(),
            nonces,
        }
    }

    #[view(getOngoingQuestTimestamp)]
    fn get_ongoing_quest_timestamp(&self, user: &ManagedAddress, id: u8) -> u64 {
        let search_result = self.ongoing_quests(user).iter().find(|q| (*q).id == id);

        let timestamp: u64 = match search_result {
            Some(q) => q.end_timestamp,
            None => 0 as u64,
        };

        timestamp
    }

    fn get_staking_rewards(&self, user: &ManagedAddress) -> BigUint {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let last_timestamp = self.last_staking_timestamp(user).get();

        if last_timestamp == 0 || current_timestamp <= last_timestamp {
            return BigUint::zero();
        }

        let block_diff: u64 = current_timestamp - last_timestamp;
        let nft_count: u64 = self.staked_nonces(user).len() as u64;

        BigUint::from(block_diff * 84 * nft_count)
    }

    fn get_token_mapper(&self, index: usize) -> FungibleTokenMapper {
        let mapper;

        if index == 0 {
            mapper = self.energy_mapper();
        } else {
            mapper = self.herbs_mapper();
        }

        mapper
    }

    fn claim_staking_rewards_for_user(&self, user: &ManagedAddress) {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let reward = self.get_staking_rewards(user);
        self.last_staking_timestamp(user).set(current_timestamp);

        if reward > 0 {
            self.energy_mapper().mint_and_send(user, reward);
        }
    }

    fn build_uris_vec(&self) -> ManagedVec<ManagedBuffer> {
        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new_from_bytes(
            "https://ipfs.io/ipfs/bafkreidiiudhpj4cy364zvucdzvtscsguybdo2q5fv7r32djgsfp3r575a".as_bytes(),
        ));

        uris
    }

    fn build_attributes_buffer(&self) -> ManagedBuffer {
        let mut attributes = ManagedBuffer::new();
        attributes.append(&ManagedBuffer::new_from_bytes(
            "tags:Company X;metadata:bafkreigrkib7uq72s2j232x3pbj34gs6sbtg2vaotv2433tsljfjevx3zu".as_bytes(),
        ));

        attributes
    }

    // Staking
    #[view(getStakedNonces)]
    #[storage_mapper("stakedNonces")]
    fn staked_nonces(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(getLastStakingTimestamp)]
    #[storage_mapper("lastStakingTimestamp")]
    fn last_staking_timestamp(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    // Tokens
    #[view(getTokenId)]
    #[storage_mapper("nonFungibleTokenMapper")]
    fn nft_mapper(&self) -> NonFungibleTokenMapper;

    #[view(getTicketsId)]
    #[storage_mapper("ticketsMapper")]
    fn tickets_mapper(&self) -> NonFungibleTokenMapper;

    #[view(getEnergyTokenId)]
    #[storage_mapper("energyMapper")]
    fn energy_mapper(&self) -> FungibleTokenMapper;

    #[view(getHerbsTokenId)]
    #[storage_mapper("herbsMapper")]
    fn herbs_mapper(&self) -> FungibleTokenMapper;

    // Quests
    #[view(getQuests)]
    #[storage_mapper("quests")]
    fn quests(&self) -> VecMapper<Quest>;

    #[view(getOngoingQuests)]
    #[storage_mapper("ongoingQuests")]
    fn ongoing_quests(&self, user: &ManagedAddress) -> VecMapper<OngoingQuest>;

    //Rewards
    #[view(getParticipantId)]
    #[storage_mapper("raffleParticipantId")]
    fn raffle_participant_id(&self, user: &ManagedAddress) -> SingleValueMapper<u16>;

    #[view(getParticipantAddress)]
    #[storage_mapper("raffleIdParticipant")]
    fn raffle_id_participant(&self, id: u16) -> SingleValueMapper<ManagedAddress>;

    #[view(getRaffleIndex)]
    #[storage_mapper("raffleIndex")]
    fn raffle_index(&self) -> SingleValueMapper<u16>;

    #[view(getRaffleVector)]
    #[storage_mapper("raffleVector")]
    fn raffle_vector(&self) -> VecMapper<u16>;

    #[view(getTestVector)]
    #[storage_mapper("testVector")]
    fn test_vector(&self) -> VecMapper<u16>;

    #[view(getTestIndex)]
    #[storage_mapper("testIndex")]
    fn test_index(&self) -> SingleValueMapper<usize>;
}
