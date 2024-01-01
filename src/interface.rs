multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/* For regular quests, the requirements/rewards slice contains the amount of tokens
in the following order: [Energy, Herbs, Gems, Essence].
For the final quest (mission), the sum of the elements in the rewards slice is equal
to the number of rewarded tickets. E.g. [1] = 1 ticket
*/
#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct Quest<M: ManagedTypeApi> {
    pub id: u8,
    pub duration: usize,
    pub is_final: bool,
    pub requirements: ManagedVec<M, u64>,
    pub rewards: ManagedVec<M, u64>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct OngoingQuest {
    pub id: u8,
    pub end_timestamp: u64,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct StakingInfo<M: ManagedTypeApi> {
    pub rewards: BigUint<M>,
    pub timestamp: u64,
    pub tokens: ManagedVec<M, Stake<M>>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct Participant<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub tickets_count: usize,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct BattleParticipant<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub tickets_count: usize,
    pub quests: usize,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct TicketStats {
    pub earners_count: usize,
    pub tickets_count: usize,
    pub most_earned: usize,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct Rarity {
    pub nonce: u16,
    pub rarity_class: u8,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct Competition {
    pub id: usize,
    pub timestamp: u64,
    pub tickets: usize,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct Airdrop<M: ManagedTypeApi> {
    pub tickets: usize,
    pub tokens: ManagedVec<M, u64>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, PartialEq)]
pub struct Stake<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub nonce: u16,
    pub amount: u16,
    pub timestamp: Option<u64>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, PartialEq)]
pub struct PlayerXp<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub xp: usize,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct LegendaryArtPage {
    pub aurora: usize,
    pub verdant: usize,
    pub solara: usize,
    pub emberheart: usize,
    pub aetheris: usize,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct StakingSummary {
    pub has_elders: bool,
    pub amount: usize,
}
