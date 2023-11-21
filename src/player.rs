const XP_THRESHOLD: usize = 3333;

multiversx_sc::imports!();

use core::iter::FromIterator;

use crate::interface::*;
use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Player: storage::Storage + helpers::Helpers {
    #[only_owner]
    #[endpoint(achievement)]
    fn achievement(&self) {}

    #[view(getPageCelestialsCustodian)]
    fn get_page_celestials_custodian(&self, user: &ManagedAddress) -> LegendaryArtPage {
        let page = LegendaryArtPage {
            aurora: self.legendary_char_aurora(user).get(),
            verdant: self.legendary_char_verdant(user).get(),
        };

        page
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

        chunk_players
    }
}
