USER_PEM="~/elrond-wallet/wallet.pem"
# USER_PEM="~/Crypto/wallet-kzcpl.pem"
PROXY="https://devnet-api.multiversx.com"
CHAIN_ID="D"

SC_ADDRESS=erd1qqqqqqqqqqqqqpgq9459wl67kadq47jq8xqwp56muskyaajvukrq9xjh8z
OWNER=erd1za7d0lzgnee39p9sytre0mss76tnht70fem0pcv0zn4undcfukrqqkzcpl

USER=erd1jvr26kvxs3xtdzapafrkupnphpzexn4zezr5lwvamam7wxqyasusjntmzr
MAC_USER=erd12vx2sn6aca5sdfhf0am7z74d4vw4yz2tm2p59nvyd3svdcqtxwnqdphu6d

TRAVELERS_ID="PTESTERS-8fd15c"
ELDERS_ID="PINKF-f70e86"

TICKETS_COLLECTION_NAME="HomeXTickets"
TICKETS_TICKER="HOMETICKET"

ENERGY_TOKEN_NAME="Energy"
ENERGY_TICKER="ENERGY"

HERBS_TOKEN_NAME="Herbs"
HERBS_TICKER="HERBS"

GEMS_TOKEN_NAME="Gems"
GEMS_TICKER="GEMS"

ESSENCE_TOKEN_NAME="Essence"
ESSENCE_TICKER="ESSENCE" 


deploy() {
    mxpy --verbose contract deploy --project=${PROJECT} --metadata-payable --metadata-payable-by-sc \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --send --outfile="deploy-devnet.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

upgrade() {
    mxpy --verbose contract upgrade ${SC_ADDRESS} --metadata-payable --metadata-payable-by-sc \
    --project=${PROJECT} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=120000000 \
    --send --outfile="upgrade-devnet.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

clear() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --function="clear"
}

setCollectionIds() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=5000000 \
    --arguments str:${TRAVELERS_ID} str:${ELDERS_ID} \
    --function="setCollectionIds"
}

issueTicketsCollection() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --value 50000000000000000 \
    --arguments str:${TICKETS_COLLECTION_NAME} str:${TICKETS_TICKER} \
    --function="issueTicketsCollection"
}

createTicketsToken() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=8000000 \
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

setGamePaused() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments true \
    --function="setGamePaused"
}

setSwappingPaused() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments false \
    --function="setSwappingPaused"
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

getTravelersCollectionId() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTravelersCollectionId"
}

getEldersCollectionId() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getEldersCollectionId"
}

getQuests() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getQuests"
}

getStakingInfo() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${MAC_USER} \
    --function="getStakingInfo"
}

setRaffleTimestamp() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments 1687269600 \
    --function="setRaffleTimestamp"
}

setRafflePot() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments 10 \
    --function="setRafflePot"
}

getRaffleVector() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getRaffleVector"
}

drawRaffleWinners() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=20000000 \
    --arguments 7 \
    --function="drawRaffleWinners"
}

getRaffleParticipants() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getRaffleParticipants"
}

getParticipants() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getParticipants"
}

getRafflePot() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getRafflePot"
}

getTxHashes() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTxHashes"
}

getStakedAddressesCount() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStakedAddressesCount"
}

getStakedNFTsCount() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStakedNFTsCount"
}

clearTicketsHistory() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=12000000 \
    --function="clearTicketsHistory"
}

clearRaffle() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=20000000 \
    --function="clearRaffle"
}

clearOngoingQuests() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=20000000 \
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

getActivePlayers() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getActivePlayers"
}