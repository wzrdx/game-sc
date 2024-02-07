multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::interface::{OngoingQuest, Quest, Stake};

#[multiversx_sc::module]
pub trait Storage {
    // Staking
    #[storage_mapper("stakedNFTs")]
    fn staked_nfts(&self, user: &ManagedAddress) -> UnorderedSetMapper<Stake<Self::Api>>;

    #[storage_mapper("lastStakingTimestamp")]
    fn last_staking_timestamp(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("stakedWallets")]
    fn staked_wallets(&self) -> UnorderedSetMapper<ManagedAddress>;

    // Rarity
    #[view(getRarityClass)]
    #[storage_mapper("travelerRarityClass")]
    fn rarity_class(&self, nonce: u64) -> SingleValueMapper<u8>;

    // NFT Collections
    #[view(getTravelersCollectionId)]
    #[storage_mapper("travelersMapper")]
    fn travelers_mapper(&self) -> NonFungibleTokenMapper;

    #[view(getEldersCollectionId)]
    #[storage_mapper("eldersMapper")]
    fn elders_mapper(&self) -> NonFungibleTokenMapper;

    // Tokens
    #[view(getTicketsId)]
    #[storage_mapper("sftTicketsMapper")]
    fn tickets_mapper(&self) -> NonFungibleTokenMapper;

    #[view(getEnergyTokenId)]
    #[storage_mapper("energyMapper")]
    fn energy_mapper(&self) -> FungibleTokenMapper;

    #[view(getHerbsTokenId)]
    #[storage_mapper("herbsMapper")]
    fn herbs_mapper(&self) -> FungibleTokenMapper;

    #[view(getGemsTokenId)]
    #[storage_mapper("gemsMapper")]
    fn gems_mapper(&self) -> FungibleTokenMapper;

    #[view(getEssenceTokenId)]
    #[storage_mapper("essenceMapper")]
    fn essence_mapper(&self) -> FungibleTokenMapper;

    // Art tokens
    #[storage_mapper("artMapper")]
    fn art_mapper(&self) -> NonFungibleTokenMapper;

    // Quests
    #[view(getQuests)]
    #[storage_mapper("quests")]
    fn quests(&self) -> VecMapper<Quest<Self::Api>>;

    #[view(getQuestsXp)]
    #[storage_mapper("questsXp")]
    fn quests_xp(&self) -> VecMapper<usize>;

    #[view(getOngoingQuests)]
    #[storage_mapper("ongoingQuests")]
    fn ongoing_quests(&self, user: &ManagedAddress) -> VecMapper<OngoingQuest>;

    // Players who have ongoing quests
    #[storage_mapper("activePlayers")]
    fn active_players(&self) -> UnorderedSetMapper<ManagedAddress>;

    // Raffles
    #[storage_mapper("rafflesCount")]
    fn raffles_count(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("raffleVectorSize")]
    fn raffle_vector_size(&self, raffle_id: usize) -> SingleValueMapper<usize>;

    #[view(getRaffleHashes)]
    #[storage_mapper("raffleHashes")]
    fn raffle_hashes(&self, raffle_id: usize) -> UnorderedSetMapper<ManagedByteArray<Self::Api, 32>>;

    #[storage_mapper("raffleTimestamp")]
    fn raffle_timestamp(&self, raffle_id: usize) -> SingleValueMapper<u64>;

    // System
    #[view(isGamePaused)]
    #[storage_mapper("isGamePaused")]
    fn is_game_paused(&self) -> SingleValueMapper<bool>;

    #[view(getArtDropTimestamp)]
    #[storage_mapper("artDropTimestamp")]
    fn art_drop_timestamp(&self) -> SingleValueMapper<u64>;

    /*
     * Elder NFT nonces are used to claim one ticket per Elder every 2 weeks.
     * After claiming rewards for a nonce, it is removed from the set.
     */
    #[storage_mapper("eldersRewardsNonces")]
    fn elders_rewards_nonces(&self) -> UnorderedSetMapper<u16>;

    // XP
    #[view(getPlayerXp)]
    #[storage_mapper("playerXp")]
    fn player_xp(&self, user: &ManagedAddress) -> SingleValueMapper<usize>;

    // Achievements
    #[storage_mapper("legendaryCharAuroraMint")]
    fn legendary_char_aurora(&self, user: &ManagedAddress) -> SingleValueMapper<usize>;

    #[storage_mapper("legendaryCharVerdantMint")]
    fn legendary_char_verdant(&self, user: &ManagedAddress) -> SingleValueMapper<usize>;

    #[storage_mapper("legendaryCharSolaraMint")]
    fn legendary_char_solara(&self, user: &ManagedAddress) -> SingleValueMapper<usize>;

    #[storage_mapper("legendaryCharEmberheartMint")]
    fn legendary_char_emberheart(&self, user: &ManagedAddress) -> SingleValueMapper<usize>;

    #[storage_mapper("legendaryCharAetherisMint")]
    fn legendary_char_aetheris(&self, user: &ManagedAddress) -> SingleValueMapper<usize>;

    // Quests Stats
    #[storage_mapper("questsTypeStats")]
    fn quests_type_stats(&self, user: &ManagedAddress, quest_type: u8) -> SingleValueMapper<usize>;

    #[storage_mapper("completedQuests")]
    fn completed_quests(&self, user: &ManagedAddress) -> SingleValueMapper<usize>;

    #[storage_mapper("ticketsEarned")]
    fn tickets_earned(&self, user: &ManagedAddress) -> SingleValueMapper<usize>;

    #[storage_mapper("energyClaimed")]
    fn energy_claimed(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint<Self::Api>>;

    // Proxies
    #[storage_mapper("scAddrAuxiliary")]
    fn sc_addr_auxiliary(&self) -> SingleValueMapper<ManagedAddress>;
}
