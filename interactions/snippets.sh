## devnet
USER_PEM="~/elrond-wallet/wallet.pem"
# USER_PEM="~/Crypto/wallets/wallet-kzcpl.pem"
PROXY="https://devnet-api.multiversx.com"
CHAIN_ID="D"
SC_ADDRESS=erd1qqqqqqqqqqqqqpgqc8s6t5594e4en4ffl60r6hn52hajkpkkukrqww29av

# OWNER=erd1za7d0lzgnee39p9sytre0mss76tnht70fem0pcv0zn4undcfukrqqkzcpl
# PLAYER=

## mainnet
# USER_PEM="~/elrond-wallet/wallet-homex.pem"
# USER_PEM="~/Crypto/wallets/wallet-homex.pem"
# PROXY="https://api.multiversx.com"
# CHAIN_ID="1"
# SC_ADDRESS=erd1qqqqqqqqqqqqqpgqpt68cy4cde6ff2wzcfsfncjv6gxjxda8dn7q9ekje9

# ART_COLLECTION_NAME="ArtOfMenhir"
# ART_TICKER="AOM"


upgrade() {
    mxpy contract build && mxpy --verbose contract upgrade ${SC_ADDRESS} --metadata-payable --metadata-payable-by-sc \
    --recall-nonce --pem=${USER_PEM} \
    --bytecode="./output/game-sc.wasm" \
    --gas-limit=115000000 \
    --send --outfile="upgrade.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

withdrawEgld() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --function="withdrawEgld"
}

setGamePaused() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments false \
    --function="setGamePaused"
}

setArtDropTimestamp() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --arguments 1704373200 \
    --function="setArtDropTimestamp"
}

refreshElderRewards() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=30000000 \
    --function="refreshElderRewards"
}

isGamePaused() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="isGamePaused"
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

getQuests() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getQuests"
}