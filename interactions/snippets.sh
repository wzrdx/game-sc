## devnet
# USER_PEM="~/elrond-wallet/wallet.pem"
# # USER_PEM="~/Crypto/wallets/wallet-kzcpl.pem"
# PROXY="https://devnet-api.multiversx.com"
# CHAIN_ID="D"
# SC_ADDRESS=erd1qqqqqqqqqqqqqpgqc8s6t5594e4en4ffl60r6hn52hajkpkkukrqww29av

# OWNER=erd1za7d0lzgnee39p9sytre0mss76tnht70fem0pcv0zn4undcfukrqqkzcpl
# PLAYER=

## mainnet
USER_PEM="~/elrond-wallet/wallet-homex.pem"
# USER_PEM="~/Crypto/wallets/wallet-homex.pem"
PROXY="https://api.multiversx.com"
CHAIN_ID="1"
SC_ADDRESS=erd1qqqqqqqqqqqqqpgqpt68cy4cde6ff2wzcfsfncjv6gxjxda8dn7q9ekje9

ART_COLLECTION_NAME="ArtOfMenhir"
ART_TICKER="AOM"


upgrade() {
    mxpy contract build && mxpy --verbose contract upgrade ${SC_ADDRESS} --metadata-payable --metadata-payable-by-sc \
    --recall-nonce --pem=${USER_PEM} \
    --bytecode="./output/game-sc.wasm" \
    --gas-limit=150000000 \
    --send --outfile="upgrade.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

airdropRaffleTraveler() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --arguments 108 2579 \
    --gas-limit=16000000 \
    --function="airdropRaffleTraveler"
}

collect() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --function="collect"
}

withdrawEgld() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --function="withdrawEgld"
}

setQuestsXp() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --arguments 10 \
    --gas-limit=32000000 \
    --function="setQuestsXp"
}

drawRaffleWinners() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=15000000 \
    --arguments 17 \
    --function="drawRaffleWinners"
}

clearRaffle() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=25000000 \
    --arguments 16 \
    --function="clearRaffle"
}

clearBattle() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --arguments 1 \
    --function="clearBattle"
}

setTrial() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=35000000 \
    --arguments 17 \
    --function="setTrial"
}

getCurrentTrial() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCurrentTrial"
}

addRaffle() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --arguments 1703505600 \
    --function="addRaffle"
}

addBattle() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --arguments 1703505600 \
    --function="addBattle"
}

copyHashes() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --function="copyHashes"
}

claimAllEnergy() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=110000000 \
    --arguments 0 30 \
    --function="claimAllEnergy"
}

setGamePaused() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments false \
    --function="setGamePaused"
}

setTrialTimestamp() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --arguments 1709283600 \
    --function="setTrialTimestamp"
}

setArtDropTimestamp() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --arguments 1704373200 \
    --function="setArtDropTimestamp"
}

getRaffleParticipants() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getRaffleParticipants"
}

getTxHashes() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTxHashes"
}

getStakedNFTsCount() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStakedNFTsCount"
}

clearOngoingQuests() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=90000000 \
    --function="clearOngoingQuests"
}

isSwappingPaused() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="isSwappingPaused"
}

isGamePaused() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="isGamePaused"
}

getUserTokenNonces() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${USER} str:${TRAVELERS_ID} \
    --function="getUserTokenNonces"
}

getStakingInfo() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${OWNER} \
    --function="getStakingInfo"
}

getStakedNFTs() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${OWNER} \
    --function="getStakedNFTs"
}

getStakedWalletsLength() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStakedWalletsLength"
}

getStakedAddressesLength() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStakedAddressesLength"
}

getCleanupAddressesCount() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getCleanupAddressesCount"
}

getQuests() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getQuests"
}

getQuestsXp() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getQuestsXp"
}

getMintedTickets() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getMintedTickets"
}

# Art Drop
issueSFTCollection() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=200000000 \
    --value 50000000000000000 \
    --arguments str:${ART_COLLECTION_NAME} str:${ART_TICKER} \
    --function="issueSFTCollection"
}

createArtToken() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=20000000 \
    --arguments 1000 \
    --function="createArtToken"
}

# Init
deploy() {
    mxpy --verbose contract deploy --metadata-payable --metadata-payable-by-sc \
    --recall-nonce --pem=${USER_PEM} \
    --bytecode="./output/game-sc.wasm" \
    --gas-limit=200000000 \
    --send --outfile="upgrade.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

setCollectionIds() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=10000000 \
    --arguments str:${TRAVELERS_ID} str:${ELDERS_ID} \
    --function="setCollectionIds"
}

issueTicketsCollection() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=200000000 \
    --value 50000000000000000 \
    --arguments str:${TICKETS_NAME} str:${TICKETS_TICKER} \
    --function="issueTicketsCollection"
}

createTicketsToken() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=120000000 \
    --arguments 1000 \
    --function="createTicketsToken"
}

issueEnergyToken() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=90000000 \
    --value 50000000000000000 \
    --arguments str:${ENERGY_TOKEN_NAME} str:${ENERGY_TICKER} \
    --function="issueEnergyToken"
}

issueHerbsToken() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=90000000 \
    --value 50000000000000000 \
    --arguments str:${HERBS_TOKEN_NAME} str:${HERBS_TICKER} \
    --function="issueHerbsToken"
}

issueGemsToken() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=90000000 \
    --value 50000000000000000 \
    --arguments str:${GEMS_TOKEN_NAME} str:${GEMS_TICKER} \
    --function="issueGemsToken"
}

issueEssenceToken() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=90000000 \
    --value 50000000000000000 \
    --arguments str:${ESSENCE_TOKEN_NAME} str:${ESSENCE_TICKER} \
    --function="issueEssenceToken"
}

getTicketsId() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTicketsId"
}

getEnergyTokenId() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getEnergyTokenId"
}

getHerbsTokenId() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getHerbsTokenId"
}

getGemsTokenId() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getGemsTokenId"
}

getEssenceTokenId() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getEssenceTokenId"
}
