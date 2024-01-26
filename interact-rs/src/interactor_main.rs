#![allow(non_snake_case)]

use game_sc::ProxyTrait as _;
use game_sc::*;

use multiversx_sc_snippets::{
    env_logger,
    erdrs::wallet::Wallet,
    multiversx_sc::{codec::multi_types::*, types::*},
    multiversx_sc_scenario::{
        api::StaticApi,
        bech32,
        scenario_format::interpret_trait::{InterpretableFrom, InterpreterContext},
        scenario_model::*,
        ContractInfo,
    },
    sdk, tokio, Interactor,
};


const GATEWAY: &str = sdk::blockchain::DEVNET_GATEWAY;
const PEM: &str = "alice.pem";
const SC_ADDRESS: &str = "";

const SYSTEM_SC_BECH32: &str = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u";
const DEFAULT_ADDRESS_EXPR: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
const TOKEN_ISSUE_COST: u64 = 50_000_000_000_000_000;

type ContractType = ContractInfo<game_sc::Proxy<StaticApi>>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut state = State::new().await;
    match cmd.as_str() {
        "deploy" => state.deploy().await,
        "upgrade" => state.upgrade().await,
        "withdrawEgld" => state.withdraw_egld().await,
        "withdrawNFTs" => state.withdraw_nfts().await,
        "burnTickets" => state.burn_tickets().await,
        "airdropResources" => state.airdrop_resources().await,
        "setTrialTimestamp" => state.set_trial_timestamp().await,
        "refreshElderRewards" => state.refresh_elder_rewards().await,
        "setGamePaused" => state.set_game_paused().await,
        "getRarityClass" => state.rarity_class().await,
        "getTravelersCollectionId" => state.travelers_mapper().await,
        "getEldersCollectionId" => state.elders_mapper().await,
        "getTicketsId" => state.tickets_mapper().await,
        "getEnergyTokenId" => state.energy_mapper().await,
        "getHerbsTokenId" => state.herbs_mapper().await,
        "getGemsTokenId" => state.gems_mapper().await,
        "getEssenceTokenId" => state.essence_mapper().await,
        "getQuests" => state.quests().await,
        "getQuestsXp" => state.quests_xp().await,
        "getOngoingQuests" => state.ongoing_quests().await,
        "getRaffleHashes" => state.raffle_hashes().await,
        "isGamePaused" => state.is_game_paused().await,
        "getArtDropTimestamp" => state.art_drop_timestamp().await,
        "getTrialTimestamp" => state.trial_timestamp().await,
        "getPlayerXp" => state.player_xp().await,
        "getRaffles" => state.get_raffles().await,
        "setQuests" => state.set_quests().await,
        "setQuestsXp" => state.set_quests_xp().await,
        "clearOngoingQuests" => state.clear_ongoing_quests().await,
        "startQuest" => state.start_quest().await,
        "startQuests" => state.start_quests().await,
        "completeQuest" => state.complete_quest().await,
        "completeAllQuests" => state.complete_all_quests().await,
        "stake" => state.stake().await,
        "unstake" => state.unstake().await,
        "claim" => state.claim().await,
        "restake" => state.restake().await,
        "claimStakingRewards" => state.claim_staking_rewards().await,
        "getStakingSummary" => state.get_staking_summary().await,
        "getStakedNFTsCount" => state.get_staked_nfts_count().await,
        "getStakingInfo" => state.get_staking_info().await,
        "getUserTokenNonces" => state.get_user_token_nonces().await,
        "getRarityClasses" => state.get_rarity_classes().await,
        "getStakedWalletsLength" => state.get_staked_wallets_length().await,
        "getStakedWallets" => state.get_staked_wallets().await,
        "claimReward" => state.claim_reward().await,
        "getElderRewards" => state.get_elder_rewards().await,
        "getPageCelestials" => state.get_page_celestials().await,
        "getXpLeaderboardSize" => state.get_xp_leaderboard_size().await,
        "getXpLeaderboard" => state.get_xp_leaderboard().await,
        "getLogSummary" => state.get_log_summary().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

struct State {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    contract: ContractType,
}

impl State {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(Wallet::from_pem_file(PEM).unwrap());
        let sc_addr_expr = if SC_ADDRESS == "" {
            DEFAULT_ADDRESS_EXPR.to_string()
        } else {
            "bech32:".to_string() + SC_ADDRESS
        };
        let contract_code = BytesValue::interpret_from(
            "file:../output/game-sc.wasm",
            &InterpreterContext::default(),
        );
        let contract = ContractType::new(sc_addr_expr);

        State {
            interactor,
            wallet_address,
            contract_code,
            contract,
        }
    }

    async fn deploy(&mut self) {
        let (new_address, _) = self
            .interactor
            .sc_deploy_get_result::<_, ()>(
                ScDeployStep::new()
                    .call(self.contract.init())
                    .from(&self.wallet_address)
                    .code(&self.contract_code)
                    .expect(TxExpect::ok().additional_error_message("deploy failed: ")),
            )
            .await;
s
        let new_address_bech32 = bech32::encode(&new_address);
        println!("new address: {new_address_bech32}");
    }

    async fn upgrade(&mut self) {
        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.upgrade())
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn withdraw_egld(&mut self) {
        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.withdraw_egld())
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn withdraw_nfts(&mut self) {
        let start = 0u32;
        let end = 0u32;
        let identifier = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.withdraw_nfts(start, end, identifier))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn burn_tickets(&mut self) {
        let amount = 0u64;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.burn_tickets(amount))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn airdrop_resources(&mut self) {
        let users = ManagedVec::from_single_item(bech32::decode(""));
        let alloc = PlaceholderInput;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.airdrop_resources(users, alloc))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn set_trial_timestamp(&mut self) {
        let timestamp = 0u64;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.set_trial_timestamp(timestamp))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn refresh_elder_rewards(&mut self) {
        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.refresh_elder_rewards())
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn set_game_paused(&mut self) {
        let value = PlaceholderInput;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.set_game_paused(value))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn rarity_class(&mut self) {
        let nonce = 0u64;

        let result_value: u8 = self
            .interactor
            .vm_query(self.contract.rarity_class(nonce))
            .await;

    }

    async fn travelers_mapper(&mut self) {
        let result_value: TokenIdentifier<StaticApi> = self
            .interactor
            .vm_query(self.contract.travelers_mapper())
            .await;

    }

    async fn elders_mapper(&mut self) {
        let result_value: TokenIdentifier<StaticApi> = self
            .interactor
            .vm_query(self.contract.elders_mapper())
            .await;

    }

    async fn tickets_mapper(&mut self) {
        let result_value: TokenIdentifier<StaticApi> = self
            .interactor
            .vm_query(self.contract.tickets_mapper())
            .await;

    }

    async fn energy_mapper(&mut self) {
        let result_value: TokenIdentifier<StaticApi> = self
            .interactor
            .vm_query(self.contract.energy_mapper())
            .await;

    }

    async fn herbs_mapper(&mut self) {
        let result_value: TokenIdentifier<StaticApi> = self
            .interactor
            .vm_query(self.contract.herbs_mapper())
            .await;

    }

    async fn gems_mapper(&mut self) {
        let result_value: TokenIdentifier<StaticApi> = self
            .interactor
            .vm_query(self.contract.gems_mapper())
            .await;

    }

    async fn essence_mapper(&mut self) {
        let result_value: TokenIdentifier<StaticApi> = self
            .interactor
            .vm_query(self.contract.essence_mapper())
            .await;

    }

    async fn quests(&mut self) {
        let result_value: MultiValueVec<Quest<StaticApi>> = self
            .interactor
            .vm_query(self.contract.quests())
            .await;

    }

    async fn quests_xp(&mut self) {
        let result_value: MultiValueVec<u32> = self
            .interactor
            .vm_query(self.contract.quests_xp())
            .await;

    }

    async fn ongoing_quests(&mut self) {
        let user = bech32::decode("");

        let result_value: MultiValueVec<OngoingQuest<StaticApi>> = self
            .interactor
            .vm_query(self.contract.ongoing_quests(user))
            .await;

    }

    async fn raffle_hashes(&mut self) {
        let raffle_id = 0u32;

        let result_value: MultiValueVec<[u8;32]> = self
            .interactor
            .vm_query(self.contract.raffle_hashes(raffle_id))
            .await;

    }

    async fn is_game_paused(&mut self) {
        let result_value: bool<StaticApi> = self
            .interactor
            .vm_query(self.contract.is_game_paused())
            .await;

    }

    async fn art_drop_timestamp(&mut self) {
        let result_value: u64 = self
            .interactor
            .vm_query(self.contract.art_drop_timestamp())
            .await;

    }

    async fn trial_timestamp(&mut self) {
        let result_value: u64 = self
            .interactor
            .vm_query(self.contract.trial_timestamp())
            .await;

    }

    async fn player_xp(&mut self) {
        let user = bech32::decode("");

        let result_value: u32 = self
            .interactor
            .vm_query(self.contract.player_xp(user))
            .await;

    }

    async fn get_raffles(&mut self) {
        let result_value: ManagedVec<StaticApi, Competition<StaticApi>> = self
            .interactor
            .vm_query(self.contract.get_raffles())
            .await;

    }

    async fn set_quests(&mut self) {
        let quests = PlaceholderInput;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.set_quests(quests))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn set_quests_xp(&mut self) {
        let values = ManagedVec::from_single_item(0u32);

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.set_quests_xp(values))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn clear_ongoing_quests(&mut self) {
        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.clear_ongoing_quests())
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn start_quest(&mut self) {
        let token_id = b"";
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let id = 0u8;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.start_quest(id))
                    .from(&self.wallet_address)
                    .esdt_transfer(token_id.to_vec(), token_nonce, token_amount)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn start_quests(&mut self) {
        let token_id = b"";
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let ids = ManagedBuffer::new_from_bytes(&b""[..]);

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.start_quests(ids))
                    .from(&self.wallet_address)
                    .esdt_transfer(token_id.to_vec(), token_nonce, token_amount)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn complete_quest(&mut self) {
        let id = 0u8;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.complete_quest(id))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn complete_all_quests(&mut self) {
        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.complete_all_quests())
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn stake(&mut self) {
        let token_id = b"";
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.stake())
                    .from(&self.wallet_address)
                    .esdt_transfer(token_id.to_vec(), token_nonce, token_amount)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn unstake(&mut self) {
        let tokens = PlaceholderInput;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.unstake(tokens))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn claim(&mut self) {
        let tokens = PlaceholderInput;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.claim(tokens))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn restake(&mut self) {
        let tokens = PlaceholderInput;

        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.restake(tokens))
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn claim_staking_rewards(&mut self) {
        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.claim_staking_rewards())
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn get_staking_summary(&mut self) {
        let wallets = ManagedVec::from_single_item(bech32::decode(""));

        let result_value: ManagedVec<StaticApi, StakingSummary<StaticApi>> = self
            .interactor
            .vm_query(self.contract.get_staking_summary(wallets))
            .await;

    }

    async fn get_staked_nfts_count(&mut self) {
        let result_value: u32 = self
            .interactor
            .vm_query(self.contract.get_staked_nfts_count())
            .await;

    }

    async fn get_staking_info(&mut self) {
        let user = bech32::decode("");

        let result_value: StakingInfo<StaticApi> = self
            .interactor
            .vm_query(self.contract.get_staking_info(user))
            .await;

    }

    async fn get_user_token_nonces(&mut self) {
        let user_address = bech32::decode("");
        let token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);

        let result_value: ManagedVec<StaticApi, u64> = self
            .interactor
            .vm_query(self.contract.get_user_token_nonces(user_address, token_id))
            .await;

    }

    async fn get_rarity_classes(&mut self) {
        let nonces = ManagedVec::from_single_item(0u16);

        let result_value: ManagedVec<StaticApi, Rarity<StaticApi>> = self
            .interactor
            .vm_query(self.contract.get_rarity_classes(nonces))
            .await;

    }

    async fn get_staked_wallets_length(&mut self) {
        let result_value: u32 = self
            .interactor
            .vm_query(self.contract.get_staked_wallets_length())
            .await;

    }

    async fn get_staked_wallets(&mut self) {
        let start = 0u32;
        let end = 0u32;

        let result_value: ManagedVec<StaticApi, ManagedAddress<StaticApi>> = self
            .interactor
            .vm_query(self.contract.get_staked_wallets(start, end))
            .await;

    }

    async fn claim_reward(&mut self) {
        let response: TypedResponse<()> = self
            .interactor
            .sc_call_use_result(
                ScCallStep::new()
                    .call(self.contract.claim_reward())
                    .from(&self.wallet_address)
                    .expect(TxExpect::ok().additional_error_message("SC call failed: ")),
            )
            .await;

        let result = response.result.unwrap();
        println!("Result: {result:?}");
    }

    async fn get_elder_rewards(&mut self) {
        let user = bech32::decode("");

        let result_value: u32 = self
            .interactor
            .vm_query(self.contract.get_elder_rewards(user))
            .await;

    }

    async fn get_page_celestials(&mut self) {
        let user = bech32::decode("");

        let result_value: LegendaryArtPage<StaticApi> = self
            .interactor
            .vm_query(self.contract.get_page_celestials(user))
            .await;

    }

    async fn get_xp_leaderboard_size(&mut self) {
        let result_value: u32 = self
            .interactor
            .vm_query(self.contract.get_xp_leaderboard_size())
            .await;

    }

    async fn get_xp_leaderboard(&mut self) {
        let start = 0u32;
        let end = 0u32;

        let result_value: ManagedVec<StaticApi, PlayerXp<StaticApi>> = self
            .interactor
            .vm_query(self.contract.get_xp_leaderboard(start, end))
            .await;

    }

    async fn get_log_summary(&mut self) {
        let user = bech32::decode("");

        let result_value: LogSummary<StaticApi> = self
            .interactor
            .vm_query(self.contract.get_log_summary(user))
            .await;

    }

}
