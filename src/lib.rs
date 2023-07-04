#![no_std]
const COMMON_ENERGY_PER_S: u64 = 278 * 3;
const UNCOMMON_ENERGY_PER_S: u64 = 278 * 3;
const RARE_ENERGY_PER_S: u64 = 278 * 3;
const ROYALS_ENERGY_PER_S: u64 = 278 * 3;
const ONEOFONE_ENERGY_PER_S: u64 = 278 * 3;

const ELDER_ENERGY_PER_S: u64 = 278 * 3;

// TODO:
// const COMMON_ENERGY_PER_S: u64 = 278 * 3;
// const UNCOMMON_ENERGY_PER_S: u64 = 278 * 4;
// const RARE_ENERGY_PER_S: u64 = 278 * 6;
// const ROYALS_ENERGY_PER_S: u64 = 278 * 8;
// const ONEOFONE_ENERGY_PER_S: u64 = 278 * 10;

// const ELDER_ENERGY_PER_S: u64 = 278 * 9;

// TODO: 4
const RAFFLE_CAP: u64 = 8;

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

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct TicketStats {
    pub earners_count: usize,
    pub tickets_count: usize,
    pub most_earned: usize,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct Rarity {
    pub nonce: u16,
    pub rarity_class: u8,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct CompactRaffle {
    pub id: usize,
    pub timestamp: u64,
    pub vector_size: usize,
}

#[multiversx_sc::contract]
pub trait GameScContract: multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule {
    #[init]
    fn init(&self) {}

    // Raffles
    #[only_owner]
    #[endpoint(addRaffle)]
    fn add_raffle(&self, timestamp: u64, participants: ManagedVec<ManagedAddress>) {
        let mut rand_source = RandomnessSource::new();
        let index = self.raffles_count().get() + 1;

        for address in participants.into_iter() {
            if self.raffle_participant_id(&address).is_empty() {
                let id: u16 = self.raffle_index().update(|i| {
                    *i += 1;
                    *i
                });

                self.raffle_participant_id(&address).set(id);
                self.raffle_id_participant(id).set(&address);
            }

            let participant_id = self.raffle_participant_id(&address).get();

            for _ in 0..rand_source.next_usize_in_range(1, 10) {
                self.raffle_vector(index).push(&participant_id);
            }

            self.raffle_participants(index).insert(address);
        }

        for hash in self.tx_hashes().iter() {
            self.raffle_hashes(index).insert(hash);
        }

        self.raffle_timestamp(index).set(timestamp);
        self.raffles_count().set(index);
    }

    #[only_owner]
    #[endpoint(copyOperatingVector)]
    fn copy_operating_vector(&self, raffle_id: usize) {
        for element in self.raffle_vector(raffle_id).into_iter() {
            self.operating_vector().push(&element);
        }
    }

    #[view(getOperatingVectorLength)]
    fn get_operating_vector_length(&self) -> usize {
        self.operating_vector().len()
    }

    // Quests
    #[only_owner]
    #[endpoint(setQuests)]
    fn set_quests(&self, quests: ManagedVec<Quest<Self::Api>>) {
        self.quests().clear();

        for quest in quests.iter() {
            self.quests().push(&quest);
        }
    }

    #[only_owner]
    #[endpoint(loadRarityClasses)]
    fn load_rarity_classes(&self, nonces: ManagedVec<u16>, classes: ManagedVec<u8>) {
        for (index, nonce) in nonces.into_iter().enumerate() {
            self.rarity_class(nonce as u64).set(classes.get(index));
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
    #[endpoint(airdropEnergy)]
    fn airdrop_energy(&self, addresses: ManagedVec<ManagedAddress>, alloc_per_addr: ManagedVec<u64>) {
        for (index, address) in addresses.into_iter().enumerate() {
            self.energy_mapper()
                .mint_and_send(&address, BigUint::from(alloc_per_addr.get(index)));
        }
    }

    #[only_owner]
    #[endpoint(clearOngoingQuests)]
    fn clear_ongoing_quests(&self) {
        for address in self.active_players().iter() {
            self.ongoing_quests(&address).clear();
        }

        self.active_players().clear();
    }

    #[only_owner]
    #[endpoint(drawRaffleWinners)]
    fn draw_raffle_winners(&self, phase: usize) {
        let mut rand_source = RandomnessSource::new();
        let mut vector: ManagedVec<u16> = ManagedVec::from_iter(self.operating_vector().iter());

        for index in 1..=5 {
            let winner_id: u16 = vector.get(rand_source.next_usize_in_range(0, vector.len()));
            let winner_address = self.raffle_id_participant(winner_id).get();

            if phase == 1 {
                if index == 1 {
                    self.send().direct_esdt(
                        &winner_address,
                        &self.travelers_mapper().get_token_id(),
                        1195 as u64,
                        &BigUint::from(1 as u32),
                    );
                } else if index == 2 {
                    self.send().direct_esdt(
                        &winner_address,
                        &self.travelers_mapper().get_token_id(),
                        468 as u64,
                        &BigUint::from(1 as u32),
                    );
                } else if index == 3 {
                    self.send().direct_esdt(
                        &winner_address,
                        &self.travelers_mapper().get_token_id(),
                        876 as u64,
                        &BigUint::from(1 as u32),
                    );
                } else if index == 4 {
                    self.send().direct_esdt(
                        &winner_address,
                        &self.travelers_mapper().get_token_id(),
                        465 as u64,
                        &BigUint::from(1 as u32),
                    );
                } else if index == 5 {
                    self.send().direct_esdt(
                        &winner_address,
                        &self.travelers_mapper().get_token_id(),
                        1959 as u64,
                        &BigUint::from(1 as u32),
                    );
                }
            } else if phase == 2 {
                if index == 1 {
                    self.send().direct_esdt(
                        &winner_address,
                        &self.travelers_mapper().get_token_id(),
                        2600 as u64,
                        &BigUint::from(1 as u32),
                    );
                } else {
                    self.send().direct_egld(&winner_address, &BigUint::from(self.to_egld(1 as u64)));
                    self.essence_mapper()
                        .mint_and_send(&winner_address, BigUint::from(100000000 as u64));
                }
            } else if phase == 3 {
                self.send().direct_egld(&winner_address, &BigUint::from(self.to_egld(1 as u64)));
                self.essence_mapper()
                    .mint_and_send(&winner_address, BigUint::from(100000000 as u64));
            } else if phase == 4 {
                if index == 1 {
                    self.send().direct_egld(&winner_address, &BigUint::from(self.to_egld(1 as u64)));
                    self.essence_mapper()
                        .mint_and_send(&winner_address, BigUint::from(100000000 as u64));
                } else {
                    self.tickets_mapper()
                        .nft_add_quantity_and_send(&winner_address, 1 as u64, BigUint::from(3 as u32));
                }
            } else if phase == 5 {
                self.tickets_mapper()
                    .nft_add_quantity_and_send(&winner_address, 1 as u64, BigUint::from(2 as u32));
            } else if phase == 6 {
                self.tickets_mapper()
                    .nft_add_quantity_and_send(&winner_address, 1 as u64, BigUint::from(2 as u32));
            }

            vector = ManagedVec::from_iter(vector.iter().filter(|id| *id != winner_id));
        }

        self.operating_vector().clear();

        for element in vector.into_iter() {
            self.operating_vector().push(&element);
        }

        let hash: ManagedByteArray<Self::Api, 32> = self.blockchain().get_tx_hash();
        self.tx_hashes_second_trial().insert(hash);
    }

    #[only_owner]
    #[endpoint(clearRaffle)]
    fn clear_raffle(&self, raffle_id: usize) {
        self.raffle_vector_size(raffle_id).set(self.raffle_vector(raffle_id).len());
        self.raffle_vector(raffle_id).clear();
        self.raffle_participants(raffle_id).clear();
        self.operating_vector().clear();
    }

    #[only_owner]
    #[endpoint(clearTicketsHistory)]
    fn clear_tickets_history(&self) {
        for address in self.tickets_earned().keys() {
            self.tickets_earned().remove(&address);
        }
    }

    #[only_owner]
    #[endpoint(setTrialTimestamp)]
    fn set_trial_timestamp(&self, timestamp: u64) {
        self.trial_timestamp().set(timestamp);
    }

    #[only_owner]
    #[endpoint(setGamePaused)]
    fn set_game_paused(&self, value: bool) {
        self.is_game_paused().set(value);
    }

    #[only_user_account]
    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
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

        self.staked_addresses().insert(caller);
    }

    #[only_user_account]
    #[endpoint(unstake)]
    fn unstake(&self, traveler_nonces: ManagedVec<u64>, elder_nonces: ManagedVec<u64>) {
        let caller = self.blockchain().get_caller();

        require!(
            (self.staked_traveler_nonces(&caller).len() + self.staked_elder_nonces(&caller).len()) > 0,
            "Must have at least one staked NFT in order to unstake"
        );

        self.claim_staking_rewards_for_user(&caller);

        let mut payments: ManagedVec<EsdtTokenPayment> = ManagedVec::new();
        let travelers_id = self.travelers_mapper().get_token_id();
        let elders_id = self.elders_mapper().get_token_id();

        for nonce in traveler_nonces.into_iter() {
            let was_removed = self.staked_traveler_nonces(&caller).swap_remove(&nonce);

            if was_removed {
                payments.push(EsdtTokenPayment::new(travelers_id.clone(), nonce, BigUint::from(1 as u32)))
            }
        }

        for nonce in elder_nonces.into_iter() {
            let was_removed = self.staked_elder_nonces(&caller).swap_remove(&nonce);

            if was_removed {
                payments.push(EsdtTokenPayment::new(elders_id.clone(), nonce, BigUint::from(1 as u32)))
            }
        }

        if payments.len() > 0 {
            self.send().direct_multi(&caller, &payments);
        }

        if self.staked_traveler_nonces(&caller).is_empty() && self.staked_elder_nonces(&caller).is_empty() {
            self.last_staking_timestamp(&caller).clear();
            self.staked_addresses().swap_remove(&caller);
        }
    }

    #[only_user_account]
    #[endpoint(claimStakingRewards)]
    fn claim_staking_rewards(&self) {
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
        self.require_conditions();
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        for ongoing_quest in self.ongoing_quests(&caller).iter() {
            require!(id != ongoing_quest.id, "Cannot start an already ongoing quest");
        }

        let quest = self.quests().get(id as usize);
        let quest_duration = quest.duration as u64;
        let trial_timestamp = self.trial_timestamp().get();

        require!(
            quest_duration + current_timestamp < trial_timestamp,
            "Quest duration exceeds end of Trial"
        );

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
        self.ongoing_quests(&caller).push(&OngoingQuest {
            id,
            end_timestamp: current_timestamp + (quest.duration as u64),
        });

        self.active_players().insert(caller.clone());
    }

    #[only_user_account]
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

        require!(current_timestamp >= ongoing_quest.end_timestamp, "Quest cannot be completed yet");

        // Rewards
        let quest = self.quests().get(id as usize);
        let rewards = &quest.rewards;

        // Ticket
        if quest.is_final {
            let tickets_amount: u64 = rewards.iter().sum();
            self.tickets_mapper()
                .nft_add_quantity_and_send(&caller, 1 as u64, BigUint::from(tickets_amount));

            let tickets_earned: usize = self.tickets_earned().remove(&caller).unwrap_or_default();
            self.tickets_earned().insert(caller.clone(), tickets_earned + 1);
        } else {
            for (i, reward) in rewards.iter().enumerate() {
                if reward > 0 {
                    let mapper = self.get_token_mapper(i);
                    mapper.mint_and_send(&caller, BigUint::from(reward));
                }
            }
        }

        self.ongoing_quests(&caller).swap_remove(index_to_remove);

        if self.ongoing_quests(&caller).len() == 0 {
            self.active_players().swap_remove(&caller);
        }
    }

    #[only_user_account]
    #[payable("*")]
    #[endpoint(joinRaffle)]
    fn join_raffle(&self, raffle_id: usize) {
        self.require_conditions();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let caller = self.blockchain().get_caller();

        require!(
            current_timestamp <= self.raffle_timestamp(raffle_id).get(),
            "Cannot submit tickets after the raffle has ended"
        );

        let payment: EsdtTokenPayment = self.call_value().single_esdt();
        self.tickets_mapper().require_same_token(&payment.token_identifier);

        let payment_amount: u64 = payment.amount.to_u64().unwrap_or_default();

        // Raffle cap
        let submitted_tickets = self.get_submitted_tickets(raffle_id, &caller);

        require!(
            (submitted_tickets as u64) + payment_amount <= RAFFLE_CAP,
            "Payment exceeds raffle tickets cap"
        );

        if self.raffle_participant_id(&caller).is_empty() {
            let id: u16 = self.raffle_index().update(|i| {
                *i += 1;
                *i
            });

            self.raffle_participant_id(&caller).set(id);
            self.raffle_id_participant(id).set(&caller);
        }

        let participant_id = self.raffle_participant_id(&caller).get();

        for _ in 0..payment_amount {
            self.raffle_vector(raffle_id).push(&participant_id);
        }

        self.raffle_participants(raffle_id).insert(caller);

        self.tickets_mapper().nft_burn(1 as u64, &payment.amount);
    }

    // Raffle
    #[view(getSubmittedTickets)]
    fn get_submitted_tickets(&self, raffle_id: usize, user: &ManagedAddress) -> usize {
        if self.raffle_participant_id(user).is_empty() {
            return 0;
        } else {
            let participant_id = self.raffle_participant_id(user).get();

            let vector = self.raffle_vector(raffle_id);
            let filter = vector.iter().filter(|t| *t == participant_id);
            return filter.count();
        }
    }

    #[view(getParticipantsCount)]
    fn get_participants_count(&self, raffle_id: usize) -> usize {
        self.raffle_participants(raffle_id).len()
    }

    #[view(getParticipants)]
    fn get_participants(&self, raffle_id: usize, start: usize, end: usize) -> ManagedVec<Participant<Self::Api>> {
        let mut participants: ManagedVec<Participant<Self::Api>> = ManagedVec::new();

        for (i, address) in self.raffle_participants(raffle_id).into_iter().enumerate() {
            if i >= start && i < end {
                participants.push(Participant {
                    address: address.clone(),
                    tickets_count: self.get_submitted_tickets(raffle_id, &address),
                });
            }
        }

        participants
    }

    #[view(getRaffles)]
    fn get_raffles(&self) -> ManagedVec<CompactRaffle> {
        let mut raffles: ManagedVec<CompactRaffle> = ManagedVec::new();
        let count = self.raffles_count().get();

        for index in 1..=count {
            let vector_size = if self.raffle_vector(index).is_empty() {
                self.raffle_vector_size(index).get()
            } else {
                self.raffle_vector(index).len()
            };

            raffles.push(CompactRaffle {
                id: index,
                timestamp: self.raffle_timestamp(index).get(),
                vector_size,
            });
        }

        raffles
    }

    // Tickets stats
    #[view(getTicketStats)]
    fn get_ticket_stats(&self) -> TicketStats {
        let mut earners_count: usize = 0;
        let mut tickets_count: usize = 0;
        let mut most_earned: usize = 0;

        for value in self.tickets_earned().values() {
            earners_count += 1;
            tickets_count += &value;
            most_earned = most_earned.max(value);
        }

        TicketStats {
            earners_count,
            tickets_count,
            most_earned,
        }
    }

    // Staking
    #[view(getStakedAddressesCount)]
    fn get_staked_addresses_count(&self) -> usize {
        self.staked_addresses().len()
    }

    #[view(getStakedNFTsCount)]
    fn get_staked_nfts_count(&self) -> usize {
        let mut count: usize = 0;

        for address in self.staked_addresses().iter() {
            count += self.staked_traveler_nonces(&address).len() + self.staked_elder_nonces(&address).len();
        }

        count
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

    #[view(getUserTokenNonces)]
    fn get_all_user_nonces(&self, user_address: ManagedAddress, token_id: TokenIdentifier) -> ManagedVec<u64> {
        let mut nonces: ManagedVec<u64> = ManagedVec::new();

        if token_id == self.travelers_mapper().get_token_id() {
            for nonce in self.staked_traveler_nonces(&user_address).iter() {
                nonces.push(nonce)
            }
        }

        if token_id == self.elders_mapper().get_token_id() {
            for nonce in self.staked_elder_nonces(&user_address).iter() {
                nonces.push(nonce)
            }
        }

        nonces
    }

    #[view(getRarityClasses)]
    fn get_rarity_classes(&self, nonces: ManagedVec<u16>) -> ManagedVec<Rarity> {
        let mut rarity_classes: ManagedVec<Rarity> = ManagedVec::new();

        for nonce in nonces.into_iter() {
            rarity_classes.push(Rarity {
                nonce,
                rarity_class: self.rarity_class(nonce as u64).get(),
            });
        }

        rarity_classes
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
        let travelers_rewards = BigUint::from(block_diff * self.get_travelers_yield(user));

        let elder_count: u64 = self.staked_elder_nonces(user).len() as u64;
        let elders_rewards = BigUint::from(block_diff * ELDER_ENERGY_PER_S * elder_count);

        travelers_rewards + elders_rewards
    }

    fn get_travelers_yield(&self, user: &ManagedAddress) -> u64 {
        let mut travelers_rewards: u64 = 0;

        for nonce in self.staked_traveler_nonces(user).iter() {
            travelers_rewards += self.get_energy_yield(self.rarity_class(nonce).get());
        }

        travelers_rewards
    }

    fn get_energy_yield(&self, rarity_class: u8) -> u64 {
        let mut energy_yield: u64 = 0;

        if rarity_class == 1 {
            energy_yield = COMMON_ENERGY_PER_S;
        } else if rarity_class == 2 {
            energy_yield = UNCOMMON_ENERGY_PER_S;
        } else if rarity_class == 3 {
            energy_yield = RARE_ENERGY_PER_S;
        } else if rarity_class == 4 {
            energy_yield = ROYALS_ENERGY_PER_S;
        } else if rarity_class == 5 {
            energy_yield = ONEOFONE_ENERGY_PER_S;
        }

        energy_yield
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

    fn require_conditions(&self) {
        require!(!self.is_game_paused().get(), "The game is temporarily paused");
    }

    fn to_egld(&self, value: u64) -> u64 {
        value.mul(1000000000000000000 as u64)
    }

    // Staking
    #[storage_mapper("stakedTravelerNonces")]
    fn staked_traveler_nonces(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[storage_mapper("stakedElderNonces")]
    fn staked_elder_nonces(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[storage_mapper("lastStakingTimestamp")]
    fn last_staking_timestamp(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("stakedAddresses")]
    fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    // Rarity
    #[view(getRarityClass)]
    #[storage_mapper("travelerRarityClass")]
    fn rarity_class(&self, nonce: u64) -> SingleValueMapper<u8>;

    // Tickets stats
    #[storage_mapper("ticketsEarned")]
    fn tickets_earned(&self) -> MapMapper<ManagedAddress, usize>;

    // NFT Collections
    #[view(getTravelersCollectionId)]
    #[storage_mapper("travelersMapper")]
    fn travelers_mapper(&self) -> NonFungibleTokenMapper;

    #[view(getEldersCollectionId)]
    #[storage_mapper("eldersMapper")]
    fn elders_mapper(&self) -> NonFungibleTokenMapper;

    // Tokens
    #[view(getTicketsId)]
    #[storage_mapper("sftTicketsMapper")]
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

    // Players who have ongoing quests
    #[storage_mapper("activePlayers")]
    fn active_players(&self) -> UnorderedSetMapper<ManagedAddress>;

    // Rewards
    #[storage_mapper("raffleParticipantId")]
    fn raffle_participant_id(&self, user: &ManagedAddress) -> SingleValueMapper<u16>;

    #[storage_mapper("raffleIdParticipant")]
    fn raffle_id_participant(&self, id: u16) -> SingleValueMapper<ManagedAddress>;

    // Participant index
    #[view(getRaffleIndex)]
    #[storage_mapper("raffleIndex")]
    fn raffle_index(&self) -> SingleValueMapper<u16>;

    #[storage_mapper("raffleVector")]
    fn raffle_vector(&self, raffle_id: usize) -> VecMapper<u16>;

    #[storage_mapper("raffleVectorSize")]
    fn raffle_vector_size(&self, raffle_id: usize) -> SingleValueMapper<usize>;

    #[storage_mapper("raffleParticipants")]
    fn raffle_participants(&self, raffle_id: usize) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getRaffleHashes)]
    #[storage_mapper("raffleHashes")]
    fn raffle_hashes(&self, raffle_id: usize) -> UnorderedSetMapper<ManagedByteArray<Self::Api, 32>>;

    #[storage_mapper("raffleTimestamp")]
    fn raffle_timestamp(&self, raffle_id: usize) -> SingleValueMapper<u64>;

    // Raffle index
    #[storage_mapper("rafflesCount")]
    fn raffles_count(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("operatingVector")]
    fn operating_vector(&self) -> VecMapper<u16>;

    // Trial 1
    #[storage_mapper("txHashes")]
    fn tx_hashes(&self) -> UnorderedSetMapper<ManagedByteArray<Self::Api, 32>>;

    // Trial 2
    #[view(getTxHashes)]
    #[storage_mapper("txHashesSecondTrial")]
    fn tx_hashes_second_trial(&self) -> UnorderedSetMapper<ManagedByteArray<Self::Api, 32>>;

    // System
    #[view(isGamePaused)]
    #[storage_mapper("isGamePaused")]
    fn is_game_paused(&self) -> SingleValueMapper<bool>;

    #[view(getTrialTimestamp)]
    #[storage_mapper("trialTimestamp")]
    fn trial_timestamp(&self) -> SingleValueMapper<u64>;
}
