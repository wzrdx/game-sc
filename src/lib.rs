#![no_std]

const TRAVELER_ENERGY_PER_S: u64 = 84; // 0.3034
const ELDER_ENERGY_PER_S: u64 = 84; // 0.3034
const START_DATE: u64 = 1686673800; // 13 June 19:30 EEST

use core::iter::FromIterator;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/* For regular quests, the requirements/rewards slice contains the amount of tokens
in the following order: [Energy, Herbs, Gems, Essence].
For the final quest (mission), the sum of the elements in the rewards slice is equal
to the number of rewarded tickets. E.g. [1] = 1 ticket
*/
#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct Quest<M: ManagedTypeApi> {
    pub id: u8,
    pub duration: usize,
    pub is_final: bool,
    pub requirements: ManagedVec<M, u64>,
    pub rewards: ManagedVec<M, u64>,
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
    pub traveler_nonces: ManagedVec<M, u64>,
    pub elder_nonces: ManagedVec<M, u64>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct Participant<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub tickets_count: usize,
}

#[multiversx_sc::contract]
pub trait GameScContract: multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(setQuests)]
    fn set_quests(&self, quests: ManagedVec<Quest<Self::Api>>) {
        self.quests().clear();

        for quest in quests.iter() {
            self.quests().push(&quest);
        }
    }

    #[only_owner]
    #[endpoint(setCollectionIds)]
    fn set_collection_ids(&self, travelers_id: TokenIdentifier, elders_id: TokenIdentifier) {
        self.travelers_mapper().set_token_id(travelers_id);
        self.elders_mapper().set_token_id(elders_id);
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

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueGemsToken)]
    fn issue_gems_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();

        self.gems_mapper()
            .issue_and_set_all_roles(issue_cost, token_display_name, token_ticker, 6 as usize, None);
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueEssenceToken)]
    fn issue_essence_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();

        self.essence_mapper()
            .issue_and_set_all_roles(issue_cost, token_display_name, token_ticker, 6 as usize, None);
    }

    #[only_owner]
    #[endpoint(faucet)]
    fn faucet(&self) {
        let caller = self.blockchain().get_caller();
        self.energy_mapper().mint_and_send(&caller, BigUint::from(10000000 as u32));
        self.herbs_mapper().mint_and_send(&caller, BigUint::from(10000000 as u32));
        self.gems_mapper().mint_and_send(&caller, BigUint::from(5000000 as u32));
        self.essence_mapper().mint_and_send(&caller, BigUint::from(5000000 as u32));

        self.tickets_mapper()
            .nft_add_quantity_and_send(&caller, 1 as u64, BigUint::from(5 as u32));
    }

    #[only_owner]
    #[endpoint(drawRaffleWinners)]
    fn draw_raffle_winners(&self, winners_count: usize) {
        require!(
            winners_count > 0 && winners_count < self.raffle_participants().len(),
            "Invalid number of winners"
        );

        let mut rand_source = RandomnessSource::new();
        let mut vector: ManagedVec<u16> = ManagedVec::from_iter(self.raffle_vector().iter());

        let mut payments_count: usize = 0;

        for _ in 0..winners_count {
            let index: usize = rand_source.next_usize_in_range(0, vector.len());
            let winner_id: u16 = vector.get(index);

            let winner_address = self.raffle_id_participant(winner_id).get();

            let payment_amount: u64 = if payments_count >= (winners_count / 2) {
                2500000000000000
            } else {
                5000000000000000
            };

            self.send().direct_egld(&winner_address, &BigUint::from(payment_amount));

            vector = ManagedVec::from_iter(vector.iter().filter(|id| *id != winner_id));
            payments_count += 1;
        }

        let hash: ManagedByteArray<Self::Api, 32> = self.blockchain().get_tx_hash();
        self.tx_hashes().insert(hash);
    }

    #[only_owner]
    #[endpoint(setRaffleTimestamp)]
    fn set_raffle_timestamp(&self, timestamp: u64) {
        self.raffle_timestamp().set(timestamp);
    }

    #[only_owner]
    #[endpoint(setRafflePot)]
    fn set_raffle_pot(&self, value: usize) {
        self.raffle_pot().set(value);
    }

    #[only_user_account]
    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        self.require_time_bounds();
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers();
        require!(payments.len() > 0, "Must stake at least one NFT");

        for payment in payments.into_iter() {
            require!(
                payment.token_identifier == self.travelers_mapper().get_token_id()
                    || payment.token_identifier == self.elders_mapper().get_token_id(),
                "NFT/s must be from the Home X collections"
            );
        }

        let caller = self.blockchain().get_caller();
        self.claim_staking_rewards_for_user(&caller);

        for payment in payments.into_iter() {
            if payment.token_identifier == self.travelers_mapper().get_token_id() {
                self.staked_traveler_nonces(&caller).insert(payment.token_nonce);
            }

            if payment.token_identifier == self.elders_mapper().get_token_id() {
                self.staked_elder_nonces(&caller).insert(payment.token_nonce);
            }
        }
    }

    #[only_user_account]
    #[endpoint(unstake)]
    fn unstake(&self) {
        self.require_time_bounds();
        let caller = self.blockchain().get_caller();

        require!(
            (self.staked_traveler_nonces(&caller).len() + self.staked_elder_nonces(&caller).len()) > 0,
            "Must have at least one staked NFT in order to unstake"
        );

        self.claim_staking_rewards_for_user(&caller);

        let mut payments: ManagedVec<EsdtTokenPayment> = ManagedVec::new();
        let travelers_id = self.travelers_mapper().get_token_id();
        let elders_id = self.elders_mapper().get_token_id();

        for nonce in self.staked_traveler_nonces(&caller).iter() {
            payments.push(EsdtTokenPayment::new(travelers_id.clone(), nonce, BigUint::from(1 as u32)))
        }

        for nonce in self.staked_elder_nonces(&caller).iter() {
            payments.push(EsdtTokenPayment::new(elders_id.clone(), nonce, BigUint::from(1 as u32)))
        }

        if payments.len() > 0 {
            self.send().direct_multi(&caller, &payments);
            self.staked_traveler_nonces(&caller).clear();
            self.staked_elder_nonces(&caller).clear();

            self.last_staking_timestamp(&caller).clear();
        }
    }

    #[only_user_account]
    #[endpoint(claimStakingRewards)]
    fn claim_staking_rewards(&self) {
        self.require_time_bounds();
        let caller = self.blockchain().get_caller();

        require!(
            (self.staked_traveler_nonces(&caller).len() + self.staked_elder_nonces(&caller).len()) > 0,
            "Must have at least one staked NFT in order to claim rewards"
        );

        self.claim_staking_rewards_for_user(&caller);
    }

    #[only_user_account]
    #[payable("*")]
    #[endpoint(startQuest)]
    fn start_quest(&self, id: u8) {
        self.require_time_bounds();
        let caller = self.blockchain().get_caller();

        for ongoing_quest in self.ongoing_quests(&caller).iter() {
            require!(id != ongoing_quest.id, "Cannot start an already ongoing quest");
        }

        let quest = self.quests().get(id as usize);

        // Payment check & burning of tokens
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers();
        let requirements = &quest.requirements;

        require!(
            payments.len() == requirements.iter().filter(|&x| x > 0).count(),
            "Received incorrect number of payments"
        );

        let mut payment_index: usize = 0;

        for (i, requirement) in requirements.iter().enumerate() {
            if requirement > 0 {
                let payment = payments.get(payment_index);
                let mapper = self.get_token_mapper(i);

                mapper.require_same_token(&payment.token_identifier);
                require!(payment.amount == BigUint::from(requirement), "Incorrect payment");
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

    #[only_user_account]
    #[endpoint(completeQuest)]
    fn complete_quest(&self, id: u8) {
        self.require_time_bounds();
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
        let rewards = &quest.rewards;

        // Ticket
        if quest.is_final {
            let tickets_amount: u64 = rewards.iter().sum();
            self.tickets_mapper()
                .nft_add_quantity_and_send(&caller, 1 as u64, BigUint::from(tickets_amount));
        } else {
            for (i, reward) in rewards.iter().enumerate() {
                if reward > 0 {
                    let mapper = self.get_token_mapper(i);
                    mapper.mint_and_send(&caller, BigUint::from(reward));
                }
            }
        }

        self.ongoing_quests(&caller).swap_remove(index_to_remove);
    }

    #[only_user_account]
    #[payable("*")]
    #[endpoint(joinRaffle)]
    fn join_raffle(&self) {
        self.require_time_bounds();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let raffle_timestamp = self.raffle_timestamp().get();

        require!(
            current_timestamp <= raffle_timestamp,
            "Cannot submit tickets after raffle submission has ended"
        );

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

        self.raffle_participants().insert(caller);

        self.tickets_mapper().nft_burn(1 as u64, &payment.amount);
    }

    #[only_user_account]
    #[payable("*")]
    #[endpoint(swapEnergy)]
    fn swap_energy(&self) {
        self.require_time_bounds();
        let payment: EsdtTokenPayment = self.call_value().single_esdt();
        self.energy_mapper().require_same_token(&payment.token_identifier);

        let caller = self.blockchain().get_caller();
        let multiplier: u64 = 1000000000;

        self.energy_mapper().burn(&payment.amount);
        self.send().direct_egld(&caller, &(payment.amount * multiplier));
    }

    #[view(getSubmittedTickets)]
    fn get_submitted_tickets(&self, user: &ManagedAddress) -> usize {
        if self.raffle_participant_id(user).is_empty() {
            return 0;
        } else {
            let participant_id = self.raffle_participant_id(user).get();
            let filtered_vec: ManagedVec<u16> =
                ManagedVec::from_iter(self.raffle_vector().iter().filter(|t| *t == participant_id));

            return filtered_vec.len();
        }
    }

    #[view(getSubmittedTicketsTotal)]
    fn get_submitted_tickets_total(&self) -> usize {
        self.raffle_vector().len()
    }

    #[view(getParticipantsCount)]
    fn get_participants_count(&self) -> usize {
        self.raffle_participants().len()
    }

    #[view(getParticipants)]
    fn get_participants(&self, start: usize, end: usize) -> ManagedVec<Participant<Self::Api>> {
        let mut participants: ManagedVec<Participant<Self::Api>> = ManagedVec::new();

        for (i, address) in self.raffle_participants().iter().enumerate() {
            if i >= start && i < end {
                participants.push(Participant {
                    address: address.clone(),
                    tickets_count: self.get_submitted_tickets(&address),
                });
            }
        }

        participants
    }

    #[view(getStakingInfo)]
    fn get_staking_info(&self, user: &ManagedAddress) -> StakingInfo<Self::Api> {
        let mut traveler_nonces: ManagedVec<u64> = ManagedVec::new();
        let mut elder_nonces: ManagedVec<u64> = ManagedVec::new();

        for nonce in self.staked_traveler_nonces(user).iter() {
            traveler_nonces.push(nonce)
        }

        for nonce in self.staked_elder_nonces(user).iter() {
            elder_nonces.push(nonce)
        }

        StakingInfo {
            rewards: self.get_staking_rewards(user),
            timestamp: self.last_staking_timestamp(user).get(),
            traveler_nonces,
            elder_nonces,
        }
    }

    fn get_token_mapper(&self, index: usize) -> FungibleTokenMapper {
        let mapper;

        if index == 0 {
            mapper = self.energy_mapper();
        } else if index == 1 {
            mapper = self.herbs_mapper();
        } else if index == 2 {
            mapper = self.gems_mapper();
        } else {
            mapper = self.essence_mapper();
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

    fn get_staking_rewards(&self, user: &ManagedAddress) -> BigUint {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let last_timestamp = self.last_staking_timestamp(user).get();

        if last_timestamp == 0 || current_timestamp <= last_timestamp {
            return BigUint::zero();
        }

        let block_diff: u64 = current_timestamp - last_timestamp;

        let traveler_count: u64 = self.staked_traveler_nonces(user).len() as u64;
        let elder_count: u64 = self.staked_elder_nonces(user).len() as u64;

        let travelers_rewards = BigUint::from(block_diff * TRAVELER_ENERGY_PER_S * traveler_count);
        let elders_rewards = BigUint::from(block_diff * ELDER_ENERGY_PER_S * elder_count);

        travelers_rewards + elders_rewards
    }

    fn build_uris_vec(&self) -> ManagedVec<ManagedBuffer> {
        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new_from_bytes(
            "https://ipfs.io/ipfs/bafybeiezcv3mihkkzoug3swhfghwoggc5i7kuocbekeyndiensdlqagoyy".as_bytes(),
        ));

        uris
    }

    fn build_attributes_buffer(&self) -> ManagedBuffer {
        let mut attributes = ManagedBuffer::new();
        attributes.append(&ManagedBuffer::new_from_bytes(
            "tags:Home X;metadata:bafkreigrkib7uq72s2j232x3pbj34gs6sbtg2vaotv2433tsljfjevx3zu".as_bytes(),
        ));

        attributes
    }

    fn require_time_bounds(&self) {
        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(current_timestamp >= START_DATE, "The game has not started yet");
    }

    // Staking
    #[storage_mapper("stakedTravelerNonces")]
    fn staked_traveler_nonces(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[storage_mapper("stakedElderNonces")]
    fn staked_elder_nonces(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(getLastStakingTimestamp)]
    #[storage_mapper("lastStakingTimestamp")]
    fn last_staking_timestamp(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    // NFT Collections
    #[view(getTravelersCollectionId)]
    #[storage_mapper("travelersMapper")]
    fn travelers_mapper(&self) -> NonFungibleTokenMapper;

    #[view(getEldersCollectionId)]
    #[storage_mapper("eldersMapper")]
    fn elders_mapper(&self) -> NonFungibleTokenMapper;

    // Tokens
    #[view(getTicketsId)]
    #[storage_mapper("ticketsMapper")]
    fn tickets_mapper(&self) -> NonFungibleTokenMapper;

    #[view(getEnergyTokenId)]
    #[storage_mapper("energyMapper")]
    fn energy_mapper(&self) -> FungibleTokenMapper;

    #[view(getHerbsTokenId)]
    #[storage_mapper("herbsMapper")]
    fn herbs_mapper(&self) -> FungibleTokenMapper;

    #[view(getGemsTokenId)]
    #[storage_mapper("gemsMapper")]
    fn gems_mapper(&self) -> FungibleTokenMapper;

    #[view(getEssenceTokenId)]
    #[storage_mapper("essenceMapper")]
    fn essence_mapper(&self) -> FungibleTokenMapper;

    // Quests
    #[view(getQuests)]
    #[storage_mapper("quests")]
    fn quests(&self) -> VecMapper<Quest<Self::Api>>;

    #[view(getOngoingQuests)]
    #[storage_mapper("ongoingQuests")]
    fn ongoing_quests(&self, user: &ManagedAddress) -> VecMapper<OngoingQuest>;

    // Rewards
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

    #[view(getRaffleParticipants)]
    #[storage_mapper("raffleParticipants")]
    fn raffle_participants(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getRaffleTimestamp)]
    #[storage_mapper("raffleTimestamp")]
    fn raffle_timestamp(&self) -> SingleValueMapper<u64>;

    #[view(getRafflePot)]
    #[storage_mapper("rafflePot")]
    fn raffle_pot(&self) -> SingleValueMapper<usize>;

    #[view(getTxHashes)]
    #[storage_mapper("txHashes")]
    fn tx_hashes(&self) -> UnorderedSetMapper<ManagedByteArray<Self::Api, 32>>;
}
