#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait GameScContract {
    #[init]
    fn init(&self, token_id: TokenIdentifier) {
        self.token_id().set_if_empty(token_id);
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();
        let nonce = payment.token_nonce;
        let token = payment.token_identifier;

        require!(
            token == self.token_id().get(),
            "Cannot stake NFT/s from different collections"
        );

        self.staked_nft(&caller).set(nonce);
    }

    #[view(getTokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getStakedNFTs)]
    #[storage_mapper("stakedNFTs")]
    fn staked_nft(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;
}
