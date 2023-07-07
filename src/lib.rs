#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod competitions;
mod helpers;
mod interface;
mod quests;
mod staking;
mod storage;

use crate::interface::*;

#[multiversx_sc::contract]
pub trait GameScContract:
    multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + storage::Storage
    + helpers::Helpers
    + competitions::Competitions
    + quests::Quests
    + staking::Staking
{
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(claimAllEnergy)]
    fn claim_all_energy(&self, start: usize, end: usize) {
        for (i, user) in self.staked_addresses().into_iter().enumerate() {
            if i >= start && i < end {
                self.claim_staking_rewards_for_user(&user);
            }
        }
    }

    #[only_owner]
    #[endpoint(airdropResources)]
    fn airdrop_resources(&self, users: ManagedVec<ManagedAddress>, alloc: ManagedVec<Airdrop<Self::Api>>) {
        for (index, address) in users.into_iter().enumerate() {
            let airdrop = alloc.get(index);

            if airdrop.tickets > 0 {
                self.tickets_mapper()
                    .nft_add_quantity_and_send(&address, 1 as u64, BigUint::from(airdrop.tickets));
            }

            for (i, amount) in airdrop.tokens.iter().enumerate() {
                if amount > 0 {
                    let mapper = self.get_token_mapper(i);
                    mapper.mint_and_send(&address, BigUint::from(amount));
                }
            }
        }
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
}
