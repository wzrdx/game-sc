#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod competitions;
mod helpers;
mod interface;
mod player;
mod quests;
mod rewards;
mod shop;
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
    + rewards::Rewards
    + shop::Shop
    + player::Player
{
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(addSpecialRole)]
    fn add_special_role(&self, sc_addr: &ManagedAddress) {
        let token_id = TokenIdentifier::from(&b"HOMETICKET-9112c2"[..]);
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(sc_addr, &token_id, (&[EsdtLocalRole::NftAddQuantity][..]).into_iter().cloned())
            .async_call()
            .call_and_exit();
    }

    #[only_owner]
    #[endpoint(transferMirageFaire)]
    fn transfer_mirage_faire(&self, sc_addr: &ManagedAddress) {
        self.tickets_mapper().nft_add_quantity_and_send(sc_addr, 1 as u64, BigUint::from(1 as usize));
    }

    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self, start: usize, end: usize, identifier: TokenIdentifier<Self::Api>) {
        let caller = self.blockchain().get_caller();

        for nonce in start..=end {
            self.send().direct_esdt(&caller, &identifier, nonce as u64, &BigUint::from(1 as u32));
        }
    }

    #[only_owner]
    #[endpoint(claimAllEnergy)]
    fn claim_all_energy(&self, start: usize, end: usize) {
        for (i, user) in self.staked_addresses().into_iter().enumerate() {
            if i >= start && i < end {
                self.claim_staking_rewards_for_user(&user);
            }
        }
    }

    // TODO: Create payments vector and send using multi
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
    #[endpoint(setTrialTimestamp)]
    fn set_trial_timestamp(&self, timestamp: u64) {
        self.trial_timestamp().set(timestamp);
    }

    #[only_owner]
    #[endpoint(setTrial)]
    fn set_trial(&self, trial: u16) {
        self.current_trial().set(trial);
        self.elders_tickets_nonces(trial - 1).clear();

        for nonce in 1..=60 {
            self.elders_tickets_nonces(trial).insert(nonce);
        }
    }

    #[only_owner]
    #[endpoint(setGamePaused)]
    fn set_game_paused(&self, value: bool) {
        self.is_game_paused().set(value);
    }
}
