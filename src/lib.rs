#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait GameScContract: multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(setTokenId)]
    fn set_token_id(&self, token_id: TokenIdentifier) {
        self.nft_mapper().set_token_id(token_id);
    }

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueStaminaToken)]
    fn issue_stamina_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();

        self.stamina_mapper()
            .issue_and_set_all_roles(issue_cost, token_display_name, token_ticker, 6 as usize, None);
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers();
        self.nft_mapper().require_all_same_token(&payments);

        let caller = self.blockchain().get_caller();

        for payment in payments.into_iter() {
            self.nonce_list(&caller).insert(payment.token_nonce);
        }
    }

    #[endpoint(unstake)]
    fn unstake(&self) {
        let caller = self.blockchain().get_caller();
        let token_id = self.nft_mapper().get_token_id();

        let mut payments: ManagedVec<EsdtTokenPayment> = ManagedVec::new();

        for nonce in self.nonce_list(&caller).iter() {
            payments.push(EsdtTokenPayment::new(token_id.clone(), nonce, BigUint::from(1 as u32)))
        }

        self.send().direct_multi(&caller, &payments);
        self.nonce_list(&caller).clear();
    }

    #[payable("*")]
    #[endpoint(exchange)]
    fn exchange(&self) {
        let caller = self.blockchain().get_caller();
        self.stamina_mapper().mint_and_send(&caller, BigUint::from(5000000 as u32));
    }

    #[payable("*")]
    #[endpoint(receive)]
    fn receive(&self) {
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers();
        require!(payments.len() == 1, "Received incorrect number of payments");

        let payment: EsdtTokenPayment = payments.get(0);

        self.stamina_mapper().require_same_token(&payment.token_identifier);

        self.stamina_mapper().burn(&payment.amount);
    }

    #[view(getStakedAmount)]
    fn get_staked_amount(&self, address: &ManagedAddress) -> usize {
        self.nonce_list(&address).len()
    }

    #[view(getNonceList)]
    #[storage_mapper("nonceList")]
    fn nonce_list(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(getTokenId)]
    #[storage_mapper("nonFungibleTokenMapper")]
    fn nft_mapper(&self) -> NonFungibleTokenMapper;

    #[view(getStaminaTokenId)]
    #[storage_mapper("staminaMapper")]
    fn stamina_mapper(&self) -> FungibleTokenMapper;
}
