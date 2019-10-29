ARGS="[{\"name\": \"tic-tac-toe-direct-hash\", \"value\": {\"bytes_value\": \"$(cat tic_tac_toe_direct_hash)\"}}, {\"name\" : \"method-name\", \"value\" : {\"string_value\" : \"move\"}}, {\"name\": \"row_position\", \"value\": {\"int_value\" : \"$1\"}}, {\"name\": \"column_position\", \"value\": {\"int_value\" : \"$2\"}}]"

./client/bin/casperlabs-client \
    --host 127.0.0.1 \
    deploy \
    --from $(cat ./data/validator-id-hex) \
    --session-name "tic-tac-toe" \
    --session-args "$ARGS" \
    --payment-amount 1 \
    --private-key ./data/validator-private.pem | \
    awk '{for(i=1;i<=NF;i++) if ($i=="Deploy") print $(i+1)}' \
    > deploy_hash

./client/bin/casperlabs-client --host 127.0.0.1 propose | \
    awk '{for(i=1;i<=NF;i++) if ($i=="Block") print $(i+1)}' \
    > block_hash

echo Deploy $(cat deploy_hash) included in block $(cat block_hash)
