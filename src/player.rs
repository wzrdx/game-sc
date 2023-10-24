const LEADERBOARD_SIZE: usize = 10;

multiversx_sc::imports!();

use multiversx_sc::types::heap::Vec;

use crate::interface::*;
use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Player: storage::Storage + helpers::Helpers {
    #[only_owner]
    #[endpoint(airdropXp)]
    fn airdrop_xp(&self) {
        let mut rand_source = RandomnessSource::new();

        for address in self.staked_addresses().into_iter() {
            self.player_xp(&address).set(rand_source.next_usize_in_range(1, 10000));
        }
    }

    #[view(getXpLeaderboard)]
    fn get_xp_leaderboard(&self) -> Vec<PlayerXp<Self::Api>> {
        let mut players: ManagedVec<PlayerXp<Self::Api>> = ManagedVec::new();

        for address in self.staked_addresses().into_iter() {
            players.push(PlayerXp {
                address: address.clone(),
                xp: self.player_xp(&address).get(),
            });
        }

        let mut vector: Vec<PlayerXp<Self::Api>> = Vec::new();

        for item in players.into_iter() {
            vector.push(item);
        }

        vector.sort_by(|a, b| b.xp.cmp(&a.xp));
        let splice: Vec<PlayerXp<Self::Api>> = vector.splice(0..LEADERBOARD_SIZE.min(vector.len()), []).collect();

        splice
    }
}
