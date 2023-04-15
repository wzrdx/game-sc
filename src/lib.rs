#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// #[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
// pub struct StakingPosition<M: ManagedTypeApi> {
//     pub nonces: ManagedVec<M, u64>,
//     pub last_timestamp: u64,
// }

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

        require!(payments.len() > 0, "Must stake at least one NFT");

        let caller = self.blockchain().get_caller();
        self.claim_staking_rewards_for_user(&caller);

        for payment in payments.into_iter() {
            self.staked_nonces(&caller).insert(payment.token_nonce);
        }
    }

    #[endpoint(unstake)]
    fn unstake(&self) {
        let caller = self.blockchain().get_caller();
        let token_id = self.nft_mapper().get_token_id();

        require!(
            self.staked_nonces(&caller).len() > 0,
            "Must have at least one staked NFT in order to unstake"
        );

        self.claim_staking_rewards_for_user(&caller);

        let mut payments: ManagedVec<EsdtTokenPayment> = ManagedVec::new();

        for nonce in self.staked_nonces(&caller).iter() {
            payments.push(EsdtTokenPayment::new(token_id.clone(), nonce, BigUint::from(1 as u32)))
        }

        if payments.len() > 0 {
            self.send().direct_multi(&caller, &payments);
            self.staked_nonces(&caller).clear();
        }
    }

    #[endpoint(claimStakingRewards)]
    fn claim_staking_rewards(&self) {
        let caller = self.blockchain().get_caller();

        require!(
            self.staked_nonces(&caller).len() > 0,
            "Must have at least one staked NFT in order to unstake"
        );

        self.claim_staking_rewards_for_user(&caller);
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

    fn claim_staking_rewards_for_user(&self, user: &ManagedAddress) {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let reward = self.get_staking_rewards(user);
        self.last_staking_timestamp(user).set(current_timestamp);

        if reward > 0 {
            self.stamina_mapper().mint_and_send(user, reward);
        }
    }

    #[view(getStakingRewards)]
    fn get_staking_rewards(&self, user: &ManagedAddress) -> BigUint {
        let current_timestamp = self.blockchain().get_block_timestamp();
        let last_timestamp = self.last_staking_timestamp(user).get();

        if last_timestamp == 0 || current_timestamp <= last_timestamp {
            return BigUint::zero();
        }

        let block_diff: u64 = current_timestamp - last_timestamp;
        let nft_count: u64 = self.staked_nonces(user).len() as u64;

        BigUint::from(block_diff * 84 * nft_count)
    }

    #[view(getStakedNonces)]
    #[storage_mapper("stakedNonces")]
    fn staked_nonces(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[view(getLastStakingTimestamp)]
    #[storage_mapper("lastStakingTimestamp")]
    fn last_staking_timestamp(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getTokenId)]
    #[storage_mapper("nonFungibleTokenMapper")]
    fn nft_mapper(&self) -> NonFungibleTokenMapper;

    #[view(getStaminaTokenId)]
    #[storage_mapper("staminaMapper")]
    fn stamina_mapper(&self) -> FungibleTokenMapper;
}
