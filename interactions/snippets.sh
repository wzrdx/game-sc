USER_PEM="~/elrond-wallet/wallet.pem"
# USER_PEM="~/Crypto/wallet-kzcpl.pem"
PROXY="https://devnet-api.multiversx.com"
CHAIN_ID="D"

SC_ADDRESS=erd1qqqqqqqqqqqqqpgq9459wl67kadq47jq8xqwp56muskyaajvukrq9xjh8z
OWNER=erd1za7d0lzgnee39p9sytre0mss76tnht70fem0pcv0zn4undcfukrqqkzcpl
USER=erd1jvr26kvxs3xtdzapafrkupnphpzexn4zezr5lwvamam7wxqyasusjntmzr
MAC_USER=erd12vx2sn6aca5sdfhf0am7z74d4vw4yz2tm2p59nvyd3svdcqtxwnqdphu6d

TOKEN_ID="HOLYCOWS-90e467"

TICKETS_COLLECTION_NAME="Tickets"
TICKETS_TICKER="TICKETS"

ENERGY_TOKEN_NAME="Energy"
ENERGY_TICKER="ENERGY"

HERBS_TOKEN_NAME="Herbs"
HERBS_TICKER="HERBS"


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
    --gas-limit=90000000 \
    --send --outfile="upgrade-devnet.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
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
    --gas-limit=60000000 \
    --value 50000000000000000 \
    --arguments str:${ENERGY_TOKEN_NAME} str:${ENERGY_TICKER} \
    --function="issueEnergyToken"
}

issueHerbsToken() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --value 50000000000000000 \
    --arguments str:${HERBS_TOKEN_NAME} str:${HERBS_TICKER} \
    --function="issueHerbsToken"
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

setTokenId() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=2000000 \
    --arguments str:${TOKEN_ID} \
    --function="setTokenId"
}

getTokenId() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTokenId"
}

getStakedNonces() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${USER} \
    --function="getStakedNonces"
}

getOngoingQuests() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${USER} \
    --function="getOngoingQuests"
}

getOngoingQuestTimestamp() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${USER} 1 \
    --function="getOngoingQuestTimestamp"
}

getParticipantId() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${USER} \
    --function="getParticipantId"
}

getRaffleIndex() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getRaffleIndex"
}

getRaffleVector() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getRaffleVector"
}

drawRaffleWinner() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=10000000 \
    --function="drawRaffleWinner"
}

getParticipantAddress() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments 2 \
    --function="getParticipantAddress"
}

getStakingInfo() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${MAC_USER} \
    --function="getStakingInfo"
}

clearOngoingQuests() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments ${USER} \
    --function="clearOngoingQuests"
}

getQuests() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getQuests"
}