multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{
    interface::OngoingQuest,
    interface::{Quest, Stake},
};

#[multiversx_sc::module]
pub trait Storage {
    // Staking
    #[storage_mapper("stakedTravelerNonces")]
    fn staked_traveler_nonces(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[storage_mapper("stakedElderNonces")]
    fn staked_elder_nonces(&self, user: &ManagedAddress) -> UnorderedSetMapper<u64>;

    #[storage_mapper("stakedNFTs")]
    fn staked_nfts(&self, user: &ManagedAddress) -> UnorderedSetMapper<Stake<Self::Api>>;

    #[storage_mapper("lastStakingTimestamp")]
    fn last_staking_timestamp(&self, user: &ManagedAddress) -> SingleValueMapper<u64>;

    // TODO: Deprecated staking system
    #[storage_mapper("stakedAddresses")]
    fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    // New staking system
    #[view(getStakedWallets)]
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
    #[view(getArtCollectionId)]
    #[storage_mapper("artMapper")]
    fn art_mapper(&self) -> NonFungibleTokenMapper;

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

    // Quests
    #[view(getQuests)]
    #[storage_mapper("quests")]
    fn quests(&self) -> VecMapper<Quest<Self::Api>>;

    #[view(getOngoingQuests)]
    #[storage_mapper("ongoingQuests")]
    fn ongoing_quests(&self, user: &ManagedAddress) -> VecMapper<OngoingQuest>;

    // Players who have ongoing quests
    #[storage_mapper("activePlayers")]
    fn active_players(&self) -> UnorderedSetMapper<ManagedAddress>;

    // Participants
    #[storage_mapper("raffleParticipantId")]
    fn raffle_participant_id(&self, user: &ManagedAddress) -> SingleValueMapper<u16>;

    #[storage_mapper("raffleIdParticipant")]
    fn raffle_id_participant(&self, id: u16) -> SingleValueMapper<ManagedAddress>;

    // Participant index
    #[view(getRaffleIndex)]
    #[storage_mapper("raffleIndex")]
    fn raffle_index(&self) -> SingleValueMapper<u16>;

    #[storage_mapper("raffleVector")]
    fn raffle_vector(&self, raffle_id: usize) -> VecMapper<u16>;

    #[storage_mapper("raffleVectorSize")]
    fn raffle_vector_size(&self, raffle_id: usize) -> SingleValueMapper<usize>;

    #[storage_mapper("raffleParticipants")]
    fn raffle_participants(&self, raffle_id: usize) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getRaffleHashes)]
    #[storage_mapper("raffleHashes")]
    fn raffle_hashes(&self, raffle_id: usize) -> UnorderedSetMapper<ManagedByteArray<Self::Api, 32>>;

    #[storage_mapper("raffleTimestamp")]
    fn raffle_timestamp(&self, raffle_id: usize) -> SingleValueMapper<u64>;

    // Raffle index
    #[storage_mapper("rafflesCount")]
    fn raffles_count(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("operatingVector")]
    fn operating_vector(&self) -> VecMapper<u16>;

    // Battles
    #[view(getBattleSubmittedTickets)]
    #[storage_mapper("battleSubmission")]
    fn battle_submission(&self, battle_id: usize, user: &ManagedAddress) -> SingleValueMapper<usize>;

    #[storage_mapper("battleParticipants")]
    fn battle_participants(&self, battle_id: usize) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("battleTotalTickets")]
    fn battle_total_tickets(&self, battle_id: usize) -> SingleValueMapper<usize>;

    #[storage_mapper("battleTimestamp")]
    fn battle_timestamp(&self, battle_id: usize) -> SingleValueMapper<u64>;

    #[view(getBattleHashes)]
    #[storage_mapper("battleHashes")]
    fn battle_hashes(&self, battle_id: usize) -> UnorderedSetMapper<ManagedByteArray<Self::Api, 32>>;

    // Can be used to compute total quests done since the first battle
    #[view(getCompletedQuests)]
    #[storage_mapper("completedQuests")]
    fn completed_quests(&self, battle_id: usize, user: &ManagedAddress) -> SingleValueMapper<usize>;

    #[storage_mapper("battlesCount")]
    fn battles_count(&self) -> SingleValueMapper<usize>;

    // System
    #[view(isGamePaused)]
    #[storage_mapper("isGamePaused")]
    fn is_game_paused(&self) -> SingleValueMapper<bool>;

    // Trial
    #[view(getTrialTimestamp)]
    #[storage_mapper("trialTimestamp")]
    fn trial_timestamp(&self) -> SingleValueMapper<u64>;

    #[view(getCurrentTrial)]
    #[storage_mapper("currentTrial")]
    fn current_trial(&self) -> SingleValueMapper<u16>;

    // Rewards
    /*
     * The Elder NFT nonces which can be used to claim tickets each Trial.
     * After claiming rewards for a nonce, it is removed from the set
     */
    #[storage_mapper("eldersTicketsNonces")]
    fn elders_tickets_nonces(&self, trial: u16) -> UnorderedSetMapper<u16>;

    // Stats
    #[view(getMintedTickets)]
    #[storage_mapper("u64mintedTickets")]
    fn minted_tickets(&self) -> SingleValueMapper<u64>;
}
