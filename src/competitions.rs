multiversx_sc::imports!();

use crate::interface::*;
use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Competitions: storage::Storage + helpers::Helpers {
    #[only_owner]
    #[endpoint(addRaffle)]
    fn add_raffle(&self, timestamp: u64) {
        let index = self.raffles_count().get() + 1;

        self.raffle_timestamp(index).set(timestamp);
        self.raffles_count().set(index);
    }

    #[only_owner]
    #[endpoint(drawRaffleWinners)]
    fn draw_raffle_winners(&self, raffle_id: usize) {
        let mut rand_source = RandomnessSource::new();

        let winner_id: u16 = self
            .raffle_vector(raffle_id)
            .get(rand_source.next_usize_in_range(1, self.raffle_vector(raffle_id).len() + 1));

        let winner_address = self.raffle_id_participant(winner_id).get();
        self.essence_mapper()
            .mint_and_send(&winner_address, BigUint::from(500_000_000 as u64));

        let hash: ManagedByteArray<Self::Api, 32> = self.blockchain().get_tx_hash();
        self.raffle_hashes(raffle_id).insert(hash);

        self.clear_raffle(raffle_id);
    }

    #[only_owner]
    #[endpoint(clearRaffle)]
    fn clear_raffle(&self, raffle_id: usize) {
        self.raffle_vector_size(raffle_id)
            .set(self.raffle_vector(raffle_id).len());
        self.raffle_vector(raffle_id).clear();
        self.raffle_participants(raffle_id).clear();
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
    fn get_raffle_participants(
        &self,
        raffle_id: usize,
        start: usize,
        end: usize,
    ) -> ManagedVec<Participant<Self::Api>> {
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
            let tickets: usize = self.raffle_vector_size(index).get() + self.raffle_vector(index).len();

            raffles.push(Competition {
                id: index,
                timestamp: self.raffle_timestamp(index).get(),
                tickets,
            });
        }

        raffles
    }
}
