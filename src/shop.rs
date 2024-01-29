multiversx_sc::imports!();

use crate::proxies;

use crate::{helpers, storage, PageAttributes};

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
    fn create_art_token(
        &self,
        name: ManagedBuffer<Self::Api>,
        cid: ManagedBuffer<Self::Api>,
        edition: ManagedBuffer<Self::Api>,
        rarity: u8,
    ) {
        let uris = ManagedVec::from_single_item(self.build_ipfs_url(cid));
        let attributes = PageAttributes { edition, rarity };

        self.send().esdt_nft_create(
            &self.art_mapper().get_token_id(),
            &BigUint::from(1 as u32),
            &name,
            &BigUint::from(1000 as u32),
            &ManagedBuffer::new(),
            &attributes,
            &uris,
        );
    }

    #[proxy]
    fn auxiliary_contract_proxy(&self, sc_address: ManagedAddress) -> proxies::auxiliary::Proxy<Self::Api>;

    #[only_owner]
    #[endpoint(access)]
    fn access(&self) {
        let sc_addr = ManagedAddress::from("");

        self.auxiliary_contract_proxy(sc_addr)
            .add(1)
            .with_gas_limit(3_000_000)
            .execute_on_dest_context();
    }

    #[view(getAttributes)]
    fn get_attributes(&self, nonce: u64, index: u64) -> ManagedBuffer {
        let attributes = self
            .art_mapper()
            .get_token_attributes::<PageAttributes<Self::Api>>(nonce);

        match index {
            1 => attributes.edition,
            2 => ManagedBuffer::from(&[attributes.rarity]),
            _ => sc_panic!("Invalid attribute index"),
        }
    }
}
