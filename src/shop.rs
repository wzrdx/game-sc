multiversx_sc::imports!();

use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Shop: multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule + storage::Storage + helpers::Helpers {
    #[only_owner]
    #[endpoint(debug)]
    fn debug(&self) {}

    #[only_user_account]
    #[payable("*")]
    #[endpoint(stub)]
    fn stub(&self) {}

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueSFTCollection)]
    fn issue_sft_collection(&self, name: ManagedBuffer, ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();

        self.art_mapper()
            .issue_and_set_all_roles(EsdtTokenType::SemiFungible, issue_cost, name, ticker, 0 as usize, None);
    }

    #[only_owner]
    #[endpoint(createArtToken)]
    fn create_art_token(&self, royalties: BigUint) {
        let uris = self.build_uris_vec();
        let buffer: ManagedBuffer = ManagedBuffer::new();

        self.send().esdt_nft_create(
            &self.art_mapper().get_token_id(),
            &BigUint::from(1 as u32),
            &ManagedBuffer::new_from_bytes("Aurora".as_bytes()),
            &royalties,
            &buffer,
            &buffer,
            &uris,
        );
    }
}
