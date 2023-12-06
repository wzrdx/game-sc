multiversx_sc::imports!();

const UNBONDING_DURATION: u64 = 604_800;

use crate::interface::*;
use crate::{helpers, storage};

use core::iter::FromIterator;

#[multiversx_sc::module]
pub trait Staking: storage::Storage + helpers::Helpers {
    #[only_user_account]
    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers();
        require!(payments.len() > 0, "Must stake at least one NFT");

        let token_ids: ManagedVec<TokenIdentifier<Self::Api>> = self.get_staking_token_ids();

        for payment in payments.into_iter() {
            require!(token_ids.contains(&payment.token_identifier), "Invalid type of token");
        }

        let caller = self.blockchain().get_caller();
        self.claim_staking_rewards_for_user(&caller);

        for payment in payments.into_iter() {
            self.staked_nfts(&caller).insert(Stake {
                token_id: payment.token_identifier,
                nonce: payment.token_nonce as u16,
                amount: payment.amount.to_u64().unwrap_or_default() as u16,
                timestamp: None,
            });
        }

        self.staked_wallets().insert(caller.clone());
    }

    #[only_user_account]
    #[endpoint(unstake)]
    fn unstake(&self, tokens: ManagedVec<Stake<Self::Api>>) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        self.claim_staking_rewards_for_user(&caller);

        for token in tokens.iter() {
            require!(token.timestamp.is_none(), "One or more tokens are already unstaked");

            let was_removed = self.staked_nfts(&caller).swap_remove(&token);
            require!(was_removed == true, "Invalid function arguments");

            let mut updated_token = token;
            updated_token.timestamp = Some(current_timestamp);

            self.staked_nfts(&caller).insert(updated_token);
        }
    }

    #[only_user_account]
    #[endpoint(claim)]
    fn claim(&self, tokens: ManagedVec<Stake<Self::Api>>) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        let mut payments: ManagedVec<EsdtTokenPayment> = ManagedVec::new();

        for token in tokens.into_iter() {
            require!(
                token.timestamp.is_some() && token.timestamp.unwrap() + UNBONDING_DURATION <= current_timestamp,
                "One or more tokens have not passed the unbonding duration"
            );

            let was_removed = self.staked_nfts(&caller).swap_remove(&token);
            require!(was_removed == true, "Invalid function arguments");

            if was_removed {
                payments.push(EsdtTokenPayment::new(token.token_id, token.nonce as u64, BigUint::from(token.amount)))
            }
        }

        if payments.len() > 0 {
            self.send().direct_multi(&caller, &payments);

            if self.staked_nfts(&caller).is_empty() {
                self.last_staking_timestamp(&caller).clear();
                self.staked_wallets().swap_remove(&caller);
            }
        }
    }

    #[only_user_account]
    #[endpoint(restake)]
    fn restake(&self, tokens: ManagedVec<Stake<Self::Api>>) {
        let caller = self.blockchain().get_caller();

        self.claim_staking_rewards_for_user(&caller);

        for token in tokens.iter() {
            require!(token.timestamp.is_some(), "One or more tokens are still staked");

            self.staked_nfts(&caller).swap_remove(&token);

            let mut updated_token = token;
            updated_token.timestamp = None;

            self.staked_nfts(&caller).insert(updated_token);
        }
    }

    #[only_user_account]
    #[endpoint(claimStakingRewards)]
    fn claim_staking_rewards(&self) {
        let caller = self.blockchain().get_caller();

        require!(
            self.staked_nfts(&caller).len() > 0,
            "Must have at least one staked NFT in order to claim rewards"
        );

        self.claim_staking_rewards_for_user(&caller);
    }

    #[view(getStakedNFTsCount)]
    fn get_staked_nfts_count(&self) -> usize {
        let mut count: usize = 0;

        for address in self.staked_wallets().iter() {
            count += self.staked_nfts(&address).iter().filter(|token| (*token).timestamp.is_none()).count();
        }

        count
    }

    #[view(getStakingInfo)]
    fn get_staking_info(&self, user: &ManagedAddress) -> StakingInfo<Self::Api> {
        let tokens: ManagedVec<Stake<Self::Api>> = ManagedVec::from_iter(self.staked_nfts(user).iter());

        StakingInfo {
            rewards: self.get_staking_rewards(user),
            timestamp: self.last_staking_timestamp(user).get(),
            tokens,
        }
    }

    #[view(getUserTokenNonces)]
    fn get_user_token_nonces(&self, user_address: ManagedAddress, token_id: TokenIdentifier) -> ManagedVec<u64> {
        let nonces: ManagedVec<u64> = self.get_staked_nonces(&user_address, token_id);
        nonces
    }

    #[view(getRarityClasses)]
    fn get_rarity_classes(&self, nonces: ManagedVec<u16>) -> ManagedVec<Rarity> {
        let mut rarity_classes: ManagedVec<Rarity> = ManagedVec::new();

        for nonce in nonces.into_iter() {
            rarity_classes.push(Rarity {
                nonce,
                rarity_class: self.rarity_class(nonce as u64).get(),
            });
        }

        rarity_classes
    }

    #[view(getStakedWalletsLength)]
    fn get_staked_wallets_length(&self) -> usize {
        self.staked_wallets().len()
    }

    #[view(isWalletStaked)]
    fn is_wallet_staked(&self, user: &ManagedAddress) -> bool {
        let is_staked: bool = self.staked_nfts(user).len() > 0;

        is_staked
    }

    fn get_staking_token_ids(&self) -> ManagedVec<TokenIdentifier<Self::Api>> {
        let mut vec: ManagedVec<TokenIdentifier<Self::Api>> = ManagedVec::new();
        vec.push(self.travelers_mapper().get_token_id());
        vec.push(self.elders_mapper().get_token_id());

        vec
    }
}
