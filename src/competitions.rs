multiversx_sc::imports!();

use crate::interface::*;
use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Competitions: storage::Storage + helpers::Helpers {
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
}
