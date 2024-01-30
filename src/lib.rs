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

mod proxies;
pub use proxies::auxiliary;

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

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint(setAddressAuxiliary)]
    fn set_address_auxiliary(&self, sc_addr: &ManagedAddress) {
        self.sc_addr_auxiliary().set(sc_addr);
    }

    #[only_owner]
    #[endpoint(upgradeProperties)]
    fn upgrade_properties(&self) {
        let token_id = self.art_mapper().get_token_id();
        let properties: TokenPropertyArguments = TokenPropertyArguments {
            can_freeze: None,
            can_wipe: None,
            can_pause: None,
            can_transfer_create_role: Some(true),
            can_mint: None,
            can_burn: None,
            can_change_owner: None,
            can_upgrade: None,
            can_add_special_roles: None,
        };

        self.send()
            .esdt_system_sc_proxy()
            .control_changes(&token_id, &properties)
            .async_call()
            .call_and_exit();
    }

    #[only_owner]
    #[endpoint(setSpecialRoles)]
    fn set_special_roles(&self, sc_addr: &ManagedAddress) {
        let token_id = self.art_mapper().get_token_id();

        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                sc_addr,
                &token_id,
                (&[EsdtLocalRole::NftAddQuantity, EsdtLocalRole::NftBurn][..])
                    .into_iter()
                    .cloned(),
            )
            .async_call()
            .call_and_exit();
    }

    #[only_owner]
    #[endpoint(transferRole)]
    fn transfer_role(&self, sc_addr: &ManagedAddress) {
        let token_id = self.art_mapper().get_token_id();

        self.send()
            .esdt_system_sc_proxy()
            .transfer_nft_create_role(&token_id, &self.blockchain().get_sc_address(), sc_addr)
            .async_call()
            .call_and_exit();
    }

    #[only_owner]
    #[endpoint(withdrawEgld)]
    fn withdraw_egld(&self) {
        let caller = self.blockchain().get_caller();
        let amount = self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0);

        self.send().direct_egld(&caller, &amount);
    }

    #[only_owner]
    #[endpoint(withdrawNFTs)]
    fn withdraw_nfts(&self, start: usize, end: usize, identifier: TokenIdentifier<Self::Api>) {
        let caller = self.blockchain().get_caller();

        for nonce in start..=end {
            self.send()
                .direct_esdt(&caller, &identifier, nonce as u64, &BigUint::from(1 as u32));
        }
    }

    #[only_owner]
    #[endpoint(burnTickets)]
    fn burn_tickets(&self, amount: u64) {
        self.tickets_mapper().nft_burn(1 as u64, &BigUint::from(amount));
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
    #[endpoint(refreshElderRewards)]
    fn refresh_elder_rewards(&self) {
        self.elders_rewards_nonces().clear();

        for nonce in 1..=60 {
            self.elders_rewards_nonces().insert(nonce);
        }
    }

    #[only_owner]
    #[endpoint(setGamePaused)]
    fn set_game_paused(&self, value: bool) {
        self.is_game_paused().set(value);
    }
}
