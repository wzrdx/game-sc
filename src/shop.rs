const ART_DROP_PRICE: usize = 3;
const ART_DROP_XP: usize = 1500;

multiversx_sc::imports!();

use crate::{helpers, storage};

#[multiversx_sc::module]
pub trait Shop: multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule + storage::Storage + helpers::Helpers {
    #[only_owner]
    #[endpoint(airdropArt)]
    fn airdrop_art(&self, wallets: ManagedVec<ManagedAddress>) {
        for address in wallets.into_iter() {
            self.art_mapper().nft_add_quantity_and_send(&address, 2 as u64, BigUint::from(1 as usize));

            self.player_xp(&address).update(|i| {
                *i += ART_DROP_XP;
            });

            self.legendary_char_verdant(&address).update(|i| {
                *i += 1;
            });
        }
    }

    #[only_owner]
    #[endpoint(setArtDropTimestamp)]
    fn set_art_drop_timestamp(&self, timestamp: u64) {
        self.art_drop_timestamp().set(timestamp);
    }

    #[only_user_account]
    #[payable("*")]
    #[endpoint(mint)]
    fn mint(&self, amount: usize) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(current_timestamp <= self.art_drop_timestamp().get(), "The art drop has ended");

        let payment: EsdtTokenPayment = self.call_value().single_esdt();
        self.tickets_mapper().require_same_token(&payment.token_identifier);

        let payment_amount: u64 = payment.amount.to_u64().unwrap_or_default();

        require!(payment_amount == (amount * ART_DROP_PRICE) as u64, "Invalid payment");

        self.tickets_mapper().nft_burn(1 as u64, &payment.amount);

        self.art_mapper().nft_add_quantity_and_send(&caller, 2 as u64, BigUint::from(amount));

        self.player_xp(&caller).update(|i| {
            *i += amount * ART_DROP_XP;
        });

        self.legendary_char_verdant(&caller).update(|i| {
            *i += amount;
        });
    }

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
            &ManagedBuffer::new_from_bytes("Verdant".as_bytes()),
            &royalties,
            &buffer,
            &buffer,
            &uris,
        );
    }
}
