#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait GameScContract {
    #[init]
    fn init(&self, token_id: TokenIdentifier) {
        self.token_id().set_if_empty(token_id);
    }

    #[only_owner]
    #[endpoint(setTokenId)]
    fn set_token_id(&self, token_id: TokenIdentifier) {
        self.my_token_id().set_token_id(token_id);
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let caller = self.blockchain().get_caller();
        let token_id = self.token_id().get();
        let payments: ManagedVec<EsdtTokenPayment> = self.call_value().all_esdt_transfers();

        for payment in payments.into_iter() {
            require!(
                payment.token_identifier == token_id,
                "Cannot stake NFT/s from other collections"
            );

            self.nonce_list(&caller).insert(payment.token_nonce);
        }
    }

    #[endpoint(unstake)]
    fn unstake(&self) {
        let caller = self.blockchain().get_caller();
        let token_id = self.token_id().get();

        let mut payments: ManagedVec<EsdtTokenPayment> = ManagedVec::new();

        for nonce in self.nonce_list(&caller).iter() {
            payments.push(EsdtTokenPayment::new(token_id.clone(), nonce, BigUint::from(1 as u32)))
        }

        self.send().direct_multi(&caller, &payments);
        self.nonce_list(&caller).clear();
    }

    #[view(getTokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getMyTokenId)]
    #[storage_mapper("myTokenId")]
    fn my_token_id(&self) -> NonFungibleTokenMapper;

    #[view(getNonceList)]
    #[storage_mapper("nonceList")]
    fn nonce_list(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;
}
