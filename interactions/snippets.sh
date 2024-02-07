# USER_PEM=$(jq -r --arg platform "$OSTYPE" '.[$platform]' "env.json")
# PROXY=$(jq -r .proxy "env.json")
# CHAIN_ID=$(jq -r .chainId "env.json")
# SC_ADDRESS=$(jq -r .address "env.json")

USER_PEM="~/Crypto/wallets/wallet-homex.pem"
PROXY="https://api.multiversx.com"
CHAIN_ID="1"
SC_ADDRESS=erd1qqqqqqqqqqqqqpgqpt68cy4cde6ff2wzcfsfncjv6gxjxda8dn7q9ekje9

# AUXILIARY=

NAME="Celestials Custodian"
CID="QmXnQtWEkjRCPQey8BNZ8BMk9JC8ZKA4FAiz4iNWC1yTf7"
EDITION="celestials"
RARITY=1


upgrade() {
    mxpy contract build && mxpy --verbose contract upgrade ${SC_ADDRESS} --metadata-payable --metadata-payable-by-sc \
    --recall-nonce --pem=${USER_PEM} \
    --bytecode="./output/game-sc.wasm" \
    --gas-limit=126000000 \
    --send --outfile="upgrade.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

upgradeProperties() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --function="upgradeProperties"
}

setSpecialRoles() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --arguments ${AUXILIARY} \
    --gas-limit=100000000 \
    --function="setSpecialRoles"
}

transferRole() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --arguments ${AUXILIARY} \
    --gas-limit=100000000 \
    --function="transferRole"
}

setAddressAuxiliary() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --arguments ${AUXILIARY} \
    --function="setAddressAuxiliary"
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
    --gas-limit=40000000 \
    --function="refreshElderRewards"
}

isGamePaused() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="isGamePaused"
}