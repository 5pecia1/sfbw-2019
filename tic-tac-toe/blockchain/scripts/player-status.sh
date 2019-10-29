./client/bin/casperlabs-client --host 127.0.0.1 query-state \
    --block-hash $(cat block_hash) \
    --key $(cat ./data/validator-id-hex) \
    --path "tic-tac-toe-direct/$1" \
    --type address
