## devnet
# USER_PEM="~/elrond-wallet/wallet.pem"
USER_PEM="~/Crypto/wallets/wallet-kzcpl.pem"
PROXY="https://devnet-api.multiversx.com"
CHAIN_ID="D"
SC_ADDRESS=erd1qqqqqqqqqqqqqpgq03qfld7ypk27r2k0wgux89573pw2htq8ukrqze9mpw

# TRAVELERS_ID="TRAVELER-51bdef"
OWNER=erd1za7d0lzgnee39p9sytre0mss76tnht70fem0pcv0zn4undcfukrqqkzcpl
OKLAMA=erd1w79cs7nv35wrkkm8cyaqjq3tkw9xvzvpcdw9prfxw05yj7lagffq5lch25
MF_ADDRESS=erd1qqqqqqqqqqqqqpgqweurk03vuhwwzsmtd334jdn6eksr0sclukrqspjv6p

# ## mainnet
# USER_PEM="~/elrond-wallet/wallet-homex.pem"
# USER_PEM="~/Crypto/wallets/wallet-homex.pem"
# PROXY="https://api.multiversx.com"
# CHAIN_ID="1"
# SC_ADDRESS=erd1qqqqqqqqqqqqqpgqpt68cy4cde6ff2wzcfsfncjv6gxjxda8dn7q9ekje9

upgrade() {
    mxpy contract build && mxpy --verbose contract upgrade ${SC_ADDRESS} --metadata-payable --metadata-payable-by-sc \
    --recall-nonce --pem=${USER_PEM} \
    --bytecode="./output/game-sc.wasm" \
    --gas-limit=140000000 \
    --send --outfile="upgrade.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

addSpecialRole() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --arguments ${MF_ADDRESS} \
    --gas-limit=600000000 \
    --function="addSpecialRole"
}

transferMirageFaire() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --arguments ${MF_ADDRESS} \
    --gas-limit=12000000 \
    --function="transferMirageFaire"
}

migrateTokens() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=32000000 \
    --function="migrateTokens"
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
    --arguments 11 \
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
    --arguments 1692892800 \
    --function="addRaffle"
}

addBattle() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=9000000 \
    --arguments 1696694400 \
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

setRaffleVectorSize() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments 2 322 \
    --function="setRaffleVectorSize"
}

getStakedAddressesLength() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStakedAddressesLength"
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

setTrialTimestamp() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=6000000 \
    --arguments 1697551200 \
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

getStakedAddresses() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStakedAddresses"
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
    --gas-limit=60000000 \
    --function="clearOngoingQuests"
}

changeBattleTimestamp() {
    mxpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --arguments 2 1696759200 \
    --gas-limit=6000000 \
    --function="changeBattleTimestamp"
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

getStakingInfo() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${OWNER} \
    --function="getStakingInfo"
}

getStakedTravelerNonces() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${OWNER} \
    --function="getStakedTravelerNonces"
}

getStakedElderNonces() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments ${OWNER} \
    --function="getStakedElderNonces"
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

getBattleParticipants() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --arguments 1 0 3 \
    --function="getBattleParticipants"
}

getMintedTickets() {
    mxpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getMintedTickets"
}

