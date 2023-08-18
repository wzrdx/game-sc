multiversx_sc::imports!();

const UNBONDING_DURATION: u64 = 900_000;

use crate::interface::*;
use crate::{helpers, storage};

use core::iter::FromIterator;

#[multiversx_sc::module]
pub trait Staking: storage::Storage + helpers::Helpers {
    #[only_owner]
    #[endpoint(migrateTokens)]
    fn migrate_tokens(&self) {
        let travelers_id = self.travelers_mapper().get_token_id();
        let elders_id = self.elders_mapper().get_token_id();

        for address in self.staked_addresses().into_iter() {
            for nonce in self.staked_traveler_nonces(&address).into_iter() {
                self.staked_nfts(&address).insert(Stake {
                    token_id: travelers_id.clone(),
                    nonce: nonce as u16,
                    amount: 1,
                    timestamp: None,
                });
            }

            for nonce in self.staked_elder_nonces(&address).into_iter() {
                self.staked_nfts(&address).insert(Stake {
                    token_id: elders_id.clone(),
                    nonce: nonce as u16,
                    amount: 1,
                    timestamp: None,
                });
            }
        }
    }

    #[only_user_account]
    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers();
        require!(payments.len() > 0, "Must stake at least one NFT");

        for payment in payments.into_iter() {
            require!(
                self.get_staking_token_ids().contains(&payment.token_identifier),
                "Invalid type of token"
            );
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

        self.staked_addresses().insert(caller);
    }

    #[only_user_account]
    #[endpoint(unstake)]
    fn unstake(&self, tokens: ManagedVec<Stake<Self::Api>>) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        let staked_tokens: ManagedVec<Stake<Self::Api>> =
            ManagedVec::from_iter(self.staked_nfts(&caller).iter().filter(|token| tokens.contains(token)));

        require!(staked_tokens.len() == tokens.len(), "Invalid function arguments");

        // Check unbonding durations
        for token in staked_tokens.iter() {
            match token.timestamp {
                Some(_timestamp) => {
                    sc_panic!("One or more tokens are already unstaked")
                }
                None => {}
            };
        }

        self.claim_staking_rewards_for_user(&caller);

        for token in staked_tokens.iter() {
            self.staked_nfts(&caller).swap_remove(&token);
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

        let staked_tokens: ManagedVec<Stake<Self::Api>> =
            ManagedVec::from_iter(self.staked_nfts(&caller).iter().filter(|token| tokens.contains(token)));

        require!(staked_tokens.len() == tokens.len(), "Invalid function arguments");

        // Check unbonding durations
        for token in staked_tokens.iter() {
            match token.timestamp {
                Some(timestamp) => {
                    require!(
                        timestamp + UNBONDING_DURATION <= current_timestamp,
                        "One or more tokens have not passed the unbonding duration"
                    );
                }
                None => sc_panic!("One or more tokens are still staked"),
            };
        }

        let mut payments: ManagedVec<EsdtTokenPayment> = ManagedVec::new();

        for token in tokens.into_iter() {
            let was_removed = self.staked_nfts(&caller).swap_remove(&token);

            if was_removed {
                payments.push(EsdtTokenPayment::new(
                    token.token_id,
                    token.nonce as u64,
                    BigUint::from(token.amount),
                ))
            }
        }

        if payments.len() > 0 {
            self.send().direct_multi(&caller, &payments);

            if self.staked_nfts(&caller).is_empty() {
                self.last_staking_timestamp(&caller).clear();
                self.staked_addresses().swap_remove(&caller);
            }
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

    // Staking
    #[view(getStakedAddressesCount)]
    fn get_staked_addresses_count(&self) -> usize {
        self.staked_addresses().len()
    }

    #[view(getStakedNFTsCount)]
    fn get_staked_nfts_count(&self) -> usize {
        let mut count: usize = 0;

        for address in self.staked_addresses().iter() {
            count += self.staked_nfts(&address).len();
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

    #[view(getStakedUsersLength)]
    fn get_staked_users_length(&self) -> usize {
        self.staked_addresses().len()
    }

    #[view(getStakedUsers)]
    fn get_staked_users(&self, start: usize, end: usize) -> ManagedVec<ManagedAddress> {
        let mut users: ManagedVec<ManagedAddress> = ManagedVec::new();

        for (i, address) in self.staked_addresses().into_iter().enumerate() {
            if i >= start && i < end {
                users.push(address);
            }
        }

        users
    }

    #[view(getStakeOfUser)]
    fn get_stake_of_user(&self, user_address: ManagedAddress) -> usize {
        self.staked_nfts(&user_address).len()
    }

    fn get_staking_token_ids(&self) -> ManagedVec<TokenIdentifier<Self::Api>> {
        let mut vec: ManagedVec<TokenIdentifier<Self::Api>> = ManagedVec::new();
        vec.push(self.travelers_mapper().get_token_id());
        vec.push(self.elders_mapper().get_token_id());

        vec
    }
}
