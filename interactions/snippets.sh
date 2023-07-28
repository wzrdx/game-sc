## devnet
# USER_PEM="~/elrond-wallet/wallet.pem"
# # USER_PEM="~/Crypto/wallets/wallet-kzcpl.pem"
# PROXY="https://devnet-api.multiversx.com"
# CHAIN_ID="D"
# SC_ADDRESS=erd1qqqqqqqqqqqqqpgq03qfld7ypk27r2k0wgux89573pw2htq8ukrqze9mpw

# TRAVELERS_ID="TRAVELER-51bdef"
# OWNER=erd1za7d0lzgnee39p9sytre0mss76tnht70fem0pcv0zn4undcfukrqqkzcpl

# ## mainnet
# USER_PEM="~/elrond-wallet/wallet-homex.pem"
USER_PEM="~/Crypto/wallets/wallet-homex.pem"
PROXY="https://api.multiversx.com"
CHAIN_ID="1"
SC_ADDRESS=erd1qqqqqqqqqqqqqpgqpt68cy4cde6ff2wzcfsfncjv6gxjxda8dn7q9ekje9


deploy() {
    mxpy --verbose contract deploy --project=${PROJECT} --metadata-payable --metadata-payable-by-sc \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --send --outfile="deploy.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

upgrade() {
    mxpy --verbose contract upgrade ${SC_ADDRESS} --metadata-payable --metadata-payable-by-sc \
    --recall-nonce --pem=${USER_PEM} \
    --bytecode="./output/game-sc.wasm" \
    --gas-limit=120000000 \
    --send --outfile="upgrade.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

drawRaffleWinners() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=30000000 \
    --arguments 9 5 \
    --function="drawRaffleWinners"
}

clearRaffle() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=36000000 \
    --arguments 9 \
    --function="clearRaffle"
}

setTrial() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=50000000 \
    --arguments 5 \
    --function="setTrial"
}

addRaffle() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --arguments 1690905600 \
    --function="addRaffle"
}

debug() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --arguments 1692455104 \
    --function="debug"
}

addBattle() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --arguments 1692455104 \
    --function="addBattle"
}

getStakedUsersLength() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStakedUsersLength"
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

setRaffleVectorSize() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments 2 322 \
    --function="setRaffleVectorSize"
}

getStakedUsersLength() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStakedUsersLength"
}

# Operating vector
copyOperatingVector() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=200000000 \
    --arguments 3 \
    --function="copyOperatingVector"
}

clearOperatingVector() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=150000000 \
    --function="clearOperatingVector"
}

getOperatingVectorLength() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getOperatingVectorLength"
}

setGamePaused() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments false \
    --function="setGamePaused"
}

# Next month 1691653497
setTrialTimestamp() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments 1690905600 \
    --function="setTrialTimestamp"
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

clearOngoingQuests() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=50000000 \
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

getRarityClass() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments 1 \
    --function="getRarityClass"
}