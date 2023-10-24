const XP_THRESHOLD: usize = 1000;

multiversx_sc::imports!();

use core::iter::FromIterator;

use crate::interface::*;
use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Player: storage::Storage + helpers::Helpers {
    #[only_owner]
    #[endpoint(airdropXp)]
    fn airdrop_xp(&self) {
        for address in self.staked_wallets().into_iter() {
            self.player_xp(&address)
                .set(self.completed_quests(1, &address).get() + self.completed_quests(2, &address).get());
        }
    }

    #[view(getXpLeaderboardSize)]
    fn get_xp_leaderboard_size(&self) -> usize {
        let mut players: ManagedVec<PlayerXp<Self::Api>> = ManagedVec::new();

        for address in self.staked_wallets().into_iter() {
            players.push(PlayerXp {
                address: address.clone(),
                xp: self.player_xp(&address).get(),
            });
        }

        players.iter().filter(|player| player.xp > XP_THRESHOLD).count()
    }

    #[view(getXpLeaderboard)]
    fn get_xp_leaderboard(&self, start: usize, end: usize) -> ManagedVec<PlayerXp<Self::Api>> {
        let mut players: ManagedVec<PlayerXp<Self::Api>> = ManagedVec::new();

        for address in self.staked_wallets().into_iter() {
            players.push(PlayerXp {
                address: address.clone(),
                xp: self.player_xp(&address).get(),
            });
        }

        let filtered_players: ManagedVec<PlayerXp<Self::Api>> = ManagedVec::from_iter(players.iter().filter(|player| player.xp > XP_THRESHOLD));
        let mut chunk_players: ManagedVec<PlayerXp<Self::Api>> = ManagedVec::new();

        for (i, player) in filtered_players.into_iter().enumerate() {
            if i >= start && i < end {
                chunk_players.push(player);
            }
        }

        // let mut vector: Vec<PlayerXp<Self::Api>> = Vec::new();

        // for item in players.into_iter() {
        //     vector.push(item);
        // }

        // vector.sort_by(|a, b| b.xp.cmp(&a.xp));
        // let splice: Vec<PlayerXp<Self::Api>> = vector.splice(0..LEADERBOARD_SIZE.min(vector.len()), []).collect();

        // splice

        chunk_players
    }
}
