multiversx_sc::imports!();

use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Shop:
    multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule + storage::Storage + helpers::Helpers
{
    // TODO:
    #[only_owner]
    #[endpoint(mint)]
    fn mint(&self, amount: usize, nonce: u64) {
        let caller = self.blockchain().get_caller();

        self.art_mapper()
            .nft_add_quantity_and_send(&caller, nonce, BigUint::from(amount));

        self.legendary_char_aetheris(&caller).update(|i| {
            *i += amount;
        });
    }
}
