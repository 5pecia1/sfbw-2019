## Assumes is running in dev-env created by install.sh
## and that a node is already running there

## deploy contract
./deploy.sh ../target/wasm32-unknown-unknown/release/tic_tac_toe.wasm

## query state to learn contract addresses
./client/bin/casperlabs-client --host 127.0.0.1 query-state \
    --block-hash $(cat block_hash) \
    --key $(cat ./data/validator-id-hex) \
    --type address | \
    grep '"tic-tac-toe"' -A 3 | \
    grep 'hash:' | \
    cut -c 16-79 > tic_tac_toe_hash

./client/bin/casperlabs-client --host 127.0.0.1 query-state \
    --block-hash $(cat block_hash) \
    --key $(cat ./data/validator-id-hex) \
    --type address | \
    grep '"tic-tac-toe-direct"' -A 3 | \
    grep 'hash:' | \
    cut -c 16-79 > tic_tac_toe_direct_hash

## Create account for second player
./client/bin/casperlabs-client --host 127.0.0.1 transfer \
    --amount 10000 \
    --target-account $(cat ./data/account-id) \
    --private-key ./data/validator-private.pem \
    --payment-amount 1 | \
    awk '{for(i=1;i<=NF;i++) if ($i=="Deploy") print $(i+1)}' \
    > deploy_hash

./client/bin/casperlabs-client --host 127.0.0.1 propose | \
    awk '{for(i=1;i<=NF;i++) if ($i=="Block") print $(i+1)}' \
    > block_hash
