multiversx_sc::imports!();

use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Rewards: storage::Storage + helpers::Helpers {
    #[only_user_account]
    #[endpoint(claimReward)]
    fn claim_reward(&self) {
        let caller = self.blockchain().get_caller();

        let mut tickets_amount: usize = 0;
        let mut available_nonces: UnorderedSetMapper<Self::Api, u16> = self.elders_tickets_nonces(self.current_trial().get());

        for nonce in self.get_staked_nonces(&caller, self.elders_mapper().get_token_id()).iter() {
            if available_nonces.contains(&(nonce as u16)) {
                tickets_amount += 1;
                available_nonces.swap_remove(&(nonce as u16));
            }
        }

        self.tickets_mapper()
            .nft_add_quantity_and_send(&caller, 1 as u64, BigUint::from(tickets_amount));
    }

    #[view(getElderRewards)]
    fn get_elder_rewards(&self, user: &ManagedAddress) -> usize {
        let mut tickets_amount: usize = 0;
        let available_nonces: UnorderedSetMapper<Self::Api, u16> = self.elders_tickets_nonces(self.current_trial().get());

        for nonce in self.get_staked_nonces(user, self.elders_mapper().get_token_id()).iter() {
            if available_nonces.contains(&(nonce as u16)) {
                tickets_amount += 1;
            }
        }

        tickets_amount
    }
}
