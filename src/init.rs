multiversx_sc::imports!();

use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Init: storage::Storage + helpers::Helpers {
    #[only_owner]
    #[endpoint(setCollectionIds)]
    fn set_collection_ids(&self, travelers_id: TokenIdentifier, elders_id: TokenIdentifier) {
        self.travelers_mapper().set_token_id(travelers_id);
        self.elders_mapper().set_token_id(elders_id);
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueTicketsCollection)]
    fn issue_tickets_collection(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value().clone_value();

        self.tickets_mapper()
            .issue_and_set_all_roles(EsdtTokenType::SemiFungible, issue_cost, token_display_name, token_ticker, 0 as usize, None);
    }

    #[only_owner]
    #[endpoint(createTicketsToken)]
    fn create_tickets_token(&self, royalties: BigUint) {
        let attributes = self.build_attributes_buffer();
        let hash_buffer = self.crypto().sha256(&attributes);
        let attributes_hash = hash_buffer.as_managed_buffer();
        let uris = self.build_uris_vec();

        self.send().esdt_nft_create(
            &self.tickets_mapper().get_token_id(),
            &BigUint::from(1 as u32),
            &ManagedBuffer::new_from_bytes("Golden Ticket".as_bytes()),
            &royalties,
            &attributes_hash,
            &attributes,
            &uris,
        );
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueEnergyToken)]
    fn issue_energy_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value().clone_value();

        self.energy_mapper()
            .issue_and_set_all_roles(issue_cost, token_display_name, token_ticker, 6 as usize, None);
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueHerbsToken)]
    fn issue_herbs_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value().clone_value();

        self.herbs_mapper()
            .issue_and_set_all_roles(issue_cost, token_display_name, token_ticker, 6 as usize, None);
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueGemsToken)]
    fn issue_gems_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value().clone_value();

        self.gems_mapper()
            .issue_and_set_all_roles(issue_cost, token_display_name, token_ticker, 6 as usize, None);
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueEssenceToken)]
    fn issue_essence_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value().clone_value();

        self.essence_mapper()
            .issue_and_set_all_roles(issue_cost, token_display_name, token_ticker, 6 as usize, None);
    }
}
