multiversx_sc::imports!();

use crate::interface::*;
use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Staking: storage::Storage + helpers::Helpers {
    #[only_user_account]
    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers();
        require!(payments.len() > 0, "Must stake at least one NFT");

        for payment in payments.into_iter() {
            require!(
                payment.token_identifier == self.travelers_mapper().get_token_id()
                    || payment.token_identifier == self.elders_mapper().get_token_id(),
                "NFT/s must be from the Home X collections"
            );
        }

        let caller = self.blockchain().get_caller();
        self.claim_staking_rewards_for_user(&caller);

        for payment in payments.into_iter() {
            if payment.token_identifier == self.travelers_mapper().get_token_id() {
                self.staked_traveler_nonces(&caller).insert(payment.token_nonce);
            }

            if payment.token_identifier == self.elders_mapper().get_token_id() {
                self.staked_elder_nonces(&caller).insert(payment.token_nonce);
            }
        }

        self.staked_addresses().insert(caller);
    }

    #[only_user_account]
    #[endpoint(unstake)]
    fn unstake(&self, traveler_nonces: ManagedVec<u64>, elder_nonces: ManagedVec<u64>) {
        let caller = self.blockchain().get_caller();

        require!(
            (self.staked_traveler_nonces(&caller).len() + self.staked_elder_nonces(&caller).len()) > 0,
            "Must have at least one staked NFT in order to unstake"
        );

        self.claim_staking_rewards_for_user(&caller);

        let mut payments: ManagedVec<EsdtTokenPayment> = ManagedVec::new();
        let travelers_id = self.travelers_mapper().get_token_id();
        let elders_id = self.elders_mapper().get_token_id();

        for nonce in traveler_nonces.into_iter() {
            let was_removed = self.staked_traveler_nonces(&caller).swap_remove(&nonce);

            if was_removed {
                payments.push(EsdtTokenPayment::new(travelers_id.clone(), nonce, BigUint::from(1 as u32)))
            }
        }

        for nonce in elder_nonces.into_iter() {
            let was_removed = self.staked_elder_nonces(&caller).swap_remove(&nonce);

            if was_removed {
                payments.push(EsdtTokenPayment::new(elders_id.clone(), nonce, BigUint::from(1 as u32)))
            }
        }

        if payments.len() > 0 {
            self.send().direct_multi(&caller, &payments);
        }

        if self.staked_traveler_nonces(&caller).is_empty() && self.staked_elder_nonces(&caller).is_empty() {
            self.last_staking_timestamp(&caller).clear();
            self.staked_addresses().swap_remove(&caller);
        }
    }

    #[only_user_account]
    #[endpoint(claimStakingRewards)]
    fn claim_staking_rewards(&self) {
        let caller = self.blockchain().get_caller();

        require!(
            (self.staked_traveler_nonces(&caller).len() + self.staked_elder_nonces(&caller).len()) > 0,
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
            count += self.staked_traveler_nonces(&address).len() + self.staked_elder_nonces(&address).len();
        }

        count
    }

    #[view(getStakingInfo)]
    fn get_staking_info(&self, user: &ManagedAddress) -> StakingInfo<Self::Api> {
        let mut traveler_nonces: ManagedVec<u64> = ManagedVec::new();
        let mut elder_nonces: ManagedVec<u64> = ManagedVec::new();

        for nonce in self.staked_traveler_nonces(user).iter() {
            traveler_nonces.push(nonce)
        }

        for nonce in self.staked_elder_nonces(user).iter() {
            elder_nonces.push(nonce)
        }

        StakingInfo {
            rewards: self.get_staking_rewards(user),
            timestamp: self.last_staking_timestamp(user).get(),
            traveler_nonces,
            elder_nonces,
        }
    }

    #[view(getUserTokenNonces)]
    fn get_all_user_nonces(&self, user_address: ManagedAddress, token_id: TokenIdentifier) -> ManagedVec<u64> {
        let mut nonces: ManagedVec<u64> = ManagedVec::new();

        if token_id == self.travelers_mapper().get_token_id() {
            for nonce in self.staked_traveler_nonces(&user_address).iter() {
                nonces.push(nonce)
            }
        }

        if token_id == self.elders_mapper().get_token_id() {
            for nonce in self.staked_elder_nonces(&user_address).iter() {
                nonces.push(nonce)
            }
        }

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
}
