### devnet
# USER_PEM="~/elrond-wallet/wallet.pem"
# USER_PEM="~/Crypto/wallet-kzcpl.pem"
# PROXY="https://devnet-api.multiversx.com"
# CHAIN_ID="D"

# SC_ADDRESS=erd1qqqqqqqqqqqqqpgq03qfld7ypk27r2k0wgux89573pw2htq8ukrqze9mpw

TRAVELERS_ID="TRAVELER-51bdef"

### mainnet
# USER_PEM="~/elrond-wallet/wallet-homex.pem"
USER_PEM="~/Crypto/wallet/wallet-homex.pem"
PROXY="https://api.multiversx.com"
CHAIN_ID="1"
SC_ADDRESS=erd1qqqqqqqqqqqqqpgqpt68cy4cde6ff2wzcfsfncjv6gxjxda8dn7q9ekje9

deploy() {
    mxpy --verbose contract deploy --project=${PROJECT} --metadata-payable --metadata-payable-by-sc \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
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

drawRaffleWinners() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=600000000 \
    --arguments 6 \
    --function="drawRaffleWinners"
}

# Raffle
copyVector() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=200000000 \
    --function="copyVector"
}

clearOperatingVector() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=200000000 \
    --function="clearOperatingVector"
}

getOperatingVectorLength() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getOperatingVectorLength"
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
    --gas-limit=200000000 \
    --value 50000000000000000 \
    --arguments str:${TICKETS_COLLECTION_NAME} str:${TICKETS_TICKER} \
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

setRaffleTimestamp() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments 1687190400 \
    --function="setRaffleTimestamp"
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
    --gas-limit=600000000 \
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

getRaffleTimestamp() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getRaffleTimestamp"
}

getUserTokenNonces() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${USER} str:${TRAVELERS_ID} \
    --function="getUserTokenNonces"
}

