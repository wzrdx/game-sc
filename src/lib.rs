#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait GameScContract {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(setTokenId)]
    fn set_token_id(&self, token_id: TokenIdentifier) {
        self.nft_mapper().set_token_id(token_id);
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

    #[view(getTokenId)]
    #[storage_mapper("nonFungibleTokenMapper")]
    fn nft_mapper(&self) -> NonFungibleTokenMapper;

    #[view(getNonceList)]
    #[storage_mapper("nonceList")]
    fn nonce_list(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;
}
