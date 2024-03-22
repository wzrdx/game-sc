USER_PEM=$(jq -r --arg platform "$OSTYPE" '.[$platform]' "env.json")
PROXY=$(jq -r .proxy "env.json")
CHAIN_ID=$(jq -r .chainId "env.json")
SC_ADDRESS=$(jq -r .address "env.json")


upgrade() {
    mxpy contract build && mxpy --verbose contract upgrade ${SC_ADDRESS} --metadata-payable --metadata-payable-by-sc \
    --recall-nonce --pem=${USER_PEM} \
    --bytecode="./output/game-sc.wasm" \
    --gas-limit=136000000 \
    --send --outfile="upgrade.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

setDoubleXpTimestamp() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=8000000 \
    --arguments 1711808880 \
    --function="setDoubleXpTimestamp"
}

debug() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=8000000 \
    --function="debug"
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