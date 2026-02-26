# game-sc

`game-sc` is a **MultiversX smart contract** that powers an on-chain game loop around:
- NFT staking,
- questing and XP progression,
- raffle-style competitions,
- periodic rewards,
- and player-oriented analytics views.

The contract is implemented in Rust (`no_std`) using `multiversx-sc` and split into focused modules (`staking`, `quests`, `competitions`, `rewards`, `player`, etc.).

## Core capabilities

### 1) Staking
Players can stake supported NFTs and manage staking positions.

Main endpoints/views:
- `stake`
- `unstake`
- `claim`
- `restake`
- `claimStakingRewards`
- `getStakingSummary`
- `getStakingInfo`
- `getStakedNFTsCount`
- `isWalletStaked`
- `getRarityClasses`

### 2) Quests & XP
The contract supports quest configuration and quest lifecycle execution.

Main endpoints:
- `setQuests` (owner)
- `setQuestsXp` (owner)
- `setDoubleXpTimestamp` (owner)
- `startQuest`
- `startQuests`
- `completeQuest`
- `completeAllQuests`

Quest payloads include requirements/rewards vectors and quest metadata (see `Quest` in `src/interface.rs`).

### 3) Competitions (Raffles)
The contract supports raffle creation, ticket submissions, winner selection, and cleanup.

Main endpoints/views:
- `addRaffle` (owner)
- `drawRaffleWinners` (owner)
- `clearRaffle` (owner)
- `joinRaffle`
- `getRaffleSubmittedTickets`
- `getRaffleParticipantsCount`
- `getRaffleParticipants`
- `getRaffles`

### 4) Rewards
Additional reward logic includes Elder reward claims and owner utilities.

Main endpoints/views:
- `claimReward`
- `getElderRewards`
- `refreshElderRewards` (owner)

### 5) Admin / Operations
Owner-only operational controls and treasury actions:
- `mintTickets`
- `burnTickets`
- `airdropResources`
- `withdrawEgld`
- `withdrawNFTs`
- `setGamePaused`

## Repository layout

```text
.
├── src/
│   ├── lib.rs              # contract composition + admin endpoints
│   ├── interface.rs        # shared data structures (Quest, Stake, PlayerInfo, ...)
│   ├── storage.rs          # storage mappers/views
│   ├── staking.rs          # staking flow
│   ├── quests.rs           # quest lifecycle
│   ├── competitions.rs     # raffle lifecycle
│   ├── rewards.rs          # reward claims
│   ├── player.rs           # leaderboard/player summary views
│   └── shop.rs             # shop module placeholder
├── interactions/
│   └── snippets.sh         # sample mxpy commands for common ops
├── meta/                   # contract metadata crate
└── tests/
    └── empty_rust_test.rs  # scenario test scaffold
```

## Prerequisites

- Rust toolchain (stable)
- `mxpy` / MultiversX SDK CLI tooling
- `jq` (used by `interactions/snippets.sh`)

## Build

```bash
mxpy contract build
```

Alternative (Rust-level):

```bash
cargo build
```

## Test

```bash
cargo test
```

> Note: `tests/empty_rust_test.rs` is currently scaffolded/commented, so you may want to add scenario coverage for critical gameplay flows.

## Local interaction workflow

The helper script `interactions/snippets.sh` includes example commands for:
- upgrade,
- raffle management,
- game pause toggles,
- and reward refresh.

Typical usage pattern:
1. Create/update `env.json` with chain, proxy, address, and pem references.
2. Source the script.
3. Call one of the helper functions.

```bash
cd interactions
source snippets.sh
setGamePaused
```

## Notes for integrators

- Several views are pagination-style (`start`, `end`) and should be queried in chunks for large data sets.
- Some gameplay data is tracked through per-user storage plus global indices/sets (`activePlayers`, `stakedWallets`, raffle maps/vectors).
- `shop.rs` currently exposes a module trait without endpoints; it is a natural extension point for marketplace/shop mechanics.

## License

No explicit license file is currently included in this repository.
