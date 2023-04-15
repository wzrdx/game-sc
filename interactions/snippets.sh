USER_PEM="~/elrond-wallet/wallet.pem"
# USER_PEM="~/Crypto/wallet-owner.pem"
PROXY="https://devnet-api.multiversx.com"
CHAIN_ID="D"

SC_ADDRESS=erd1qqqqqqqqqqqqqpgq9459wl67kadq47jq8xqwp56muskyaajvukrq9xjh8z
OWNER=erd1za7d0lzgnee39p9sytre0mss76tnht70fem0pcv0zn4undcfukrqqkzcpl
USER=erd1jvr26kvxs3xtdzapafrkupnphpzexn4zezr5lwvamam7wxqyasusjntmzr

TOKEN_ID="HOLYCOWS-90e467"

STAMINA_TOKEN_NAME="Stamina"
STAMINA_TICKER="STAMINA"


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

issueStaminaToken() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=60000000 \
    --value 50000000000000000 \
    --arguments str:${STAMINA_TOKEN_NAME} str:${STAMINA_TICKER} \
    --function="issueStaminaToken"
}

getStaminaTokenId() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStaminaTokenId"
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

getStakingRewards() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${USER} \
    --function="getStakingRewards"
}
