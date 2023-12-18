const RAFFLE_CAP: u64 = 5;

multiversx_sc::imports!();

use crate::interface::*;
use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Competitions: storage::Storage + helpers::Helpers {
    #[only_owner]
    #[endpoint(changeBattleTimestamp)]
    fn change_battle_timestamp(&self, index: usize, timestamp: u64) {
        self.battle_timestamp(index).set(timestamp);
    }

    #[only_owner]
    #[endpoint(addBattle)]
    fn add_battle(&self, timestamp: u64) {
        let index = self.battles_count().get() + 1;

        self.battle_timestamp(index).set(timestamp);
        self.battles_count().set(index);
    }

    #[only_owner]
    #[endpoint(addRaffle)]
    fn add_raffle(&self, timestamp: u64) {
        let index = self.raffles_count().get() + 1;

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

    #[only_owner]
    #[endpoint(drawRaffleWinners)]
    fn draw_raffle_winners(&self, raffle_id: usize) {
        let mut rand_source = RandomnessSource::new();

        let winner_id: u16 = self
            .raffle_vector(raffle_id)
            .get(rand_source.next_usize_in_range(1, self.raffle_vector(raffle_id).len() + 1));

        let winner_address = self.raffle_id_participant(winner_id).get();

        // if let 1..=5 = raffle_id
        if let 101..=105 = raffle_id {
            self.send().direct_egld(&winner_address, &BigUint::from(500_000_000_000_000_000 as u64));
        }

        let hash: ManagedByteArray<Self::Api, 32> = self.blockchain().get_tx_hash();
        self.raffle_hashes(raffle_id).insert(hash);

        self.clear_raffle(raffle_id);
    }

    #[only_owner]
    #[endpoint(airdropRaffleTraveler)]
    fn airdrop_raffle_traveler(&self, raffle_id: usize, nonce: u64) {
        let mut rand_source = RandomnessSource::new();

        let winner_id: u16 = self
            .raffle_vector(raffle_id)
            .get(rand_source.next_usize_in_range(1, self.raffle_vector(raffle_id).len() + 1));

        let winner_address = self.raffle_id_participant(winner_id).get();

        self.send()
            .direct_esdt(&winner_address, &self.travelers_mapper().get_token_id(), nonce, &BigUint::from(1 as u32));

        let hash: ManagedByteArray<Self::Api, 32> = self.blockchain().get_tx_hash();
        self.raffle_hashes(raffle_id).insert(hash);

        self.clear_raffle(raffle_id);
    }

    #[only_owner]
    #[endpoint(clearRaffle)]
    fn clear_raffle(&self, raffle_id: usize) {
        self.raffle_vector_size(raffle_id).set(self.raffle_vector(raffle_id).len());
        self.raffle_vector(raffle_id).clear();
        self.raffle_participants(raffle_id).clear();
    }

    #[only_owner]
    #[endpoint(airdropBattlePrizes)]
    fn airdrop_battle_prizes(&self, battle_id: usize, winners: ManagedVec<ManagedAddress<Self::Api>>) {
        let cpa = TokenIdentifier::from(&b"CPA-76d979"[..]);

        for (i, address) in winners.into_iter().enumerate() {
            if i == 0 {
                self.send_nft(&address, &cpa, 413);
                self.send().direct_egld(&address, &BigUint::from(500_000_000_000_000_000 as u64));
                self.essence_mapper().mint_and_send(&address, BigUint::from(800_000_000 as u64));
            } else if i == 1 {
                self.send_nft(&address, &cpa, 489);
                self.send().direct_egld(&address, &BigUint::from(300_000_000_000_000_000 as u64));
                self.essence_mapper().mint_and_send(&address, BigUint::from(500_000_000 as u64));
            } else if i == 2 {
                self.send().direct_egld(&address, &BigUint::from(500_000_000_000_000_000 as u64));
                self.essence_mapper().mint_and_send(&address, BigUint::from(300_000_000 as u64));
            } else if i == 3 {
                self.send().direct_egld(&address, &BigUint::from(400_000_000_000_000_000 as u64));
                self.essence_mapper().mint_and_send(&address, BigUint::from(200_000_000 as u64));
            } else if i == 4 {
                self.send().direct_egld(&address, &BigUint::from(300_000_000_000_000_000 as u64));
                self.essence_mapper().mint_and_send(&address, BigUint::from(100_000_000 as u64));
            }
        }

        let hash: ManagedByteArray<Self::Api, 32> = self.blockchain().get_tx_hash();
        self.battle_hashes(battle_id).insert(hash);
    }

    #[only_owner]
    #[endpoint(clearBattle)]
    fn clear_battle(&self, battle_id: usize) {
        for participant in self.battle_participants(battle_id).into_iter() {
            self.battle_submission(battle_id, &participant).clear();
        }

        self.battle_participants(battle_id).clear();
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
        let submitted_tickets = self.get_raffle_submitted_tickets(raffle_id, &caller);

        require!((submitted_tickets as u64) + payment_amount <= RAFFLE_CAP, "Payment exceeds raffle tickets cap");

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

    #[only_user_account]
    #[payable("*")]
    #[endpoint(joinBattle)]
    fn join_battle(&self, battle_id: usize) {
        self.require_conditions();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let caller = self.blockchain().get_caller();

        require!(
            current_timestamp <= self.battle_timestamp(battle_id).get(),
            "Cannot submit tickets after the battle has ended"
        );

        let payment: EsdtTokenPayment = self.call_value().single_esdt();
        self.tickets_mapper().require_same_token(&payment.token_identifier);

        let payment_amount: u64 = payment.amount.to_u64().unwrap_or_default();

        self.battle_submission(battle_id, &caller).update(|i| {
            *i += payment_amount as usize;
        });

        self.battle_total_tickets(battle_id).update(|i| {
            *i += payment_amount as usize;
        });

        self.battle_participants(battle_id).insert(caller);
        self.tickets_mapper().nft_burn(1 as u64, &payment.amount);
    }

    // Operating vector
    #[view(getOperatingVectorLength)]
    fn get_operating_vector_length(&self) -> usize {
        self.operating_vector().len()
    }

    // Raffles
    #[view(getRaffleSubmittedTickets)]
    fn get_raffle_submitted_tickets(&self, raffle_id: usize, user: &ManagedAddress) -> usize {
        if self.raffle_participant_id(user).is_empty() {
            return 0;
        } else {
            let participant_id = self.raffle_participant_id(user).get();

            let vector = self.raffle_vector(raffle_id);
            let filter = vector.iter().filter(|t| *t == participant_id);
            return filter.count();
        }
    }

    #[view(getRaffleParticipantsCount)]
    fn get_raffle_participants_count(&self, raffle_id: usize) -> usize {
        self.raffle_participants(raffle_id).len()
    }

    #[view(getRaffleParticipants)]
    fn get_raffle_participants(&self, raffle_id: usize, start: usize, end: usize) -> ManagedVec<Participant<Self::Api>> {
        let mut participants: ManagedVec<Participant<Self::Api>> = ManagedVec::new();

        for (i, address) in self.raffle_participants(raffle_id).into_iter().enumerate() {
            if i >= start && i < end {
                participants.push(Participant {
                    address: address.clone(),
                    tickets_count: self.get_raffle_submitted_tickets(raffle_id, &address),
                });
            }
        }

        participants
    }

    #[view(getRaffles)]
    fn get_raffles(&self) -> ManagedVec<Competition> {
        let mut raffles: ManagedVec<Competition> = ManagedVec::new();
        let count = self.raffles_count().get();

        for index in 1..=count {
            let tickets = if self.raffle_vector(index).is_empty() {
                self.raffle_vector_size(index).get()
            } else {
                self.raffle_vector(index).len()
            };

            raffles.push(Competition {
                id: index,
                timestamp: self.raffle_timestamp(index).get(),
                tickets,
            });
        }

        raffles
    }

    // Battles
    #[view(getBattleParticipantsCount)]
    fn get_battle_participants_count(&self, battle_id: usize) -> usize {
        self.battle_participants(battle_id).len()
    }

    #[view(getBattleParticipants)]
    fn get_battle_participants(&self, battle_id: usize, start: usize, end: usize) -> ManagedVec<BattleParticipant<Self::Api>> {
        let mut participants: ManagedVec<BattleParticipant<Self::Api>> = ManagedVec::new();

        for (i, address) in self.battle_participants(battle_id).into_iter().enumerate() {
            if i >= start && i < end {
                participants.push(BattleParticipant {
                    address: address.clone(),
                    tickets_count: self.battle_submission(battle_id, &address).get(),
                    quests: self.completed_quests(battle_id, &address).get(),
                });
            }
        }

        participants
    }

    #[view(getBattles)]
    fn get_battles(&self) -> ManagedVec<Competition> {
        let mut battles: ManagedVec<Competition> = ManagedVec::new();
        let count = self.battles_count().get();

        for index in 1..=count {
            let tickets = self.battle_total_tickets(index).get();

            battles.push(Competition {
                id: index,
                timestamp: self.battle_timestamp(index).get(),
                tickets,
            });
        }

        battles
    }
}
