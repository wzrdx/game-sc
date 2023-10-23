multiversx_sc::imports!();

use crate::interface::*;
use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Player: storage::Storage + helpers::Helpers {
    #[view(getXpLeaderboard)]
    fn get_xp_leaderboard(&self) -> ManagedVec<PlayerXp<Self::Api>> {
        let mut players: ManagedVec<PlayerXp<Self::Api>> = ManagedVec::new();

        for address in self.staked_addresses().into_iter() {
            players.push(PlayerXp {
                address: address.clone(),
                xp: self.player_xp(&address).get(),
            });
        }

        players
    }
}
