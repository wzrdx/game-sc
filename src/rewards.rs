multiversx_sc::imports!();

use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Rewards: storage::Storage + helpers::Helpers {
    #[only_user_account]
    #[endpoint(claimReward)]
    fn claim_reward(&self) {
        let caller = self.blockchain().get_caller();
        let elders_id = self.elders_mapper().get_token_id();

        let mut amount: usize = 0;

        for nonce in self.get_staked_nonces(&caller, &elders_id).iter() {
            if self.elders_rewards_nonces().contains(&(nonce as u16)) {
                amount += 2;
                self.elders_rewards_nonces().swap_remove(&(nonce as u16));
            }
        }

        self.tickets_mapper()
            .nft_add_quantity_and_send(&caller, 1 as u64, BigUint::from(amount));
    }

    #[view(getElderRewards)]
    fn get_elder_rewards(&self, user: &ManagedAddress) -> usize {
        let mut tickets_amount: usize = 0;
        let elders_id = self.elders_mapper().get_token_id();

        for nonce in self.get_staked_nonces(user, &elders_id).iter() {
            if self.elders_rewards_nonces().contains(&(nonce as u16)) {
                tickets_amount += 1;
            }
        }

        tickets_amount
    }
}
