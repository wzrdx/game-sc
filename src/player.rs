const XP_THRESHOLD: usize = 14000;

multiversx_sc::imports!();

use crate::auxiliary::ProxyTrait as _;
use core::iter::FromIterator;

use crate::interface::*;
use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Player: storage::Storage + helpers::Helpers {
    #[view(getPageCelestials)]
    fn get_page_celestials(&self, user: &ManagedAddress) -> LegendaryArtPage {
        let page = LegendaryArtPage {
            aurora: self.legendary_char_aurora(user).get(),
            verdant: self.legendary_char_verdant(user).get(),
            solara: self.legendary_char_solara(user).get(),
            emberheart: self.legendary_char_emberheart(user).get(),
            aetheris: self.legendary_char_aetheris(user).get(),
        };

        page
    }

    #[view(getXpLeaderboardSize)]
    fn get_xp_leaderboard_size(&self) -> usize {
        let mut xp_values: ManagedVec<usize> = ManagedVec::new();

        for address in self.staked_wallets().into_iter() {
            xp_values.push(self.player_xp(&address).get());
        }

        xp_values.iter().filter(|xp| *xp > XP_THRESHOLD).count()
    }

    #[view(getXpLeaderboard)]
    fn get_xp_leaderboard(&self, start: usize, end: usize) -> ManagedVec<PlayerXp<Self::Api>> {
        let mut players: ManagedVec<PlayerXp<Self::Api>> = ManagedVec::new();

        for address in self.staked_wallets().into_iter() {
            let pages_minted: usize = self
                .auxiliary_contract_proxy(self.sc_addr_auxiliary().get())
                .get_pages_minted(&address)
                .execute_on_dest_context::<usize>();

            players.push(PlayerXp {
                address: address.clone(),
                xp: self.player_xp(&address).get(),
                pages_minted,
            });
        }

        let filtered_players: ManagedVec<PlayerXp<Self::Api>> =
            ManagedVec::from_iter(players.iter().filter(|player| player.xp > XP_THRESHOLD));
        let mut chunk_players: ManagedVec<PlayerXp<Self::Api>> = ManagedVec::new();

        for (i, player) in filtered_players.into_iter().enumerate() {
            if i >= start && i < end {
                chunk_players.push(player);
            }
        }

        chunk_players
    }

    #[view(getLogSummary)]
    fn get_log_summary(&self, user: &ManagedAddress) -> LogSummary {
        let summary = LogSummary {
            quests_completed: self.completed_quests(user).get(),
            type_1: self.quests_type_stats(user, 1 as u8).get(),
            type_2: self.quests_type_stats(user, 2 as u8).get(),
            type_3: self.quests_type_stats(user, 3 as u8).get(),
            tickets: self.tickets_earned(user).get(),
            energy: self.energy_claimed(user).get().to_u64().unwrap_or_default(),
        };

        summary
    }
}
