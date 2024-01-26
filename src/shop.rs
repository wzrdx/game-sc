multiversx_sc::imports!();

use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Shop:
    multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule + storage::Storage + helpers::Helpers
{
    #[only_user_account]
    #[payable("*")]
    #[endpoint(mint)]
    fn mint(&self, amount: usize, nonce: u64) {
        let caller = self.blockchain().get_caller();

        let payment: EsdtTokenPayment = self.call_value().single_esdt();
        self.tickets_mapper().require_same_token(&payment.token_identifier);

        let payment_amount: u64 = payment.amount.to_u64().unwrap_or_default();

        require!(payment_amount == 1 as u64, "Invalid payment");

        self.tickets_mapper().nft_burn(1 as u64, &payment.amount);

        self.art_mapper()
            .nft_add_quantity_and_send(&caller, nonce, BigUint::from(amount));
    }

    #[only_owner]
    #[endpoint(createArtToken)]
    fn create_art_token(&self, royalties: BigUint, name: ManagedBuffer<Self::Api>, uri: ManagedBuffer<Self::Api>) {
        let uris = ManagedVec::from_single_item(uri);
        let buffer: ManagedBuffer = ManagedBuffer::new();

        self.send().esdt_nft_create(
            &self.art_mapper().get_token_id(),
            &BigUint::from(1 as u32),
            &name,
            &royalties,
            &ManagedBuffer::new(),
            &buffer,
            &uris,
        );
    }
}
