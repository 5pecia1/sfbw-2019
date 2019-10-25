## create directory
mkdir dev-env
cd dev-env

## Get node package
wget https://github.com/CasperLabs/CasperLabs/releases/download/v0.8.1/casperlabs-node-0.8.1.tgz
tar -xf casperlabs-node-0.8.1.tgz
rm casperlabs-node-0.8.1.tgz
mv ./casperlabs-node-0.8.1/ ./node/

## Get client package
wget https://github.com/CasperLabs/CasperLabs/releases/download/v0.8.1/casperlabs-client-0.8.1.tgz
tar -xf casperlabs-client-0.8.1.tgz
rm casperlabs-client-0.8.1.tgz
mv ./casperlabs-client-0.8.1/ ./client/

## Get the execution engine package
wget https://github.com/CasperLabs/CasperLabs/releases/download/v0.8.1/casperlabs-engine-grpc-server-0.8.1_linux_x86_64.tar.gz
tar -xf casperlabs-engine-grpc-server-0.8.1_linux_x86_64.tar.gz
rm casperlabs-engine-grpc-server-0.8.1_linux_x86_64.tar.gz

## keys for node
mkdir data
wget https://raw.githubusercontent.com/CasperLabs/CasperLabs/v0.8.1/hack/key-management/docker-gen-keys.sh
chmod +x docker-gen-keys.sh
./docker-gen-keys.sh ./data/
rm docker-gen-keys.sh

## Configurations for genesis block
mkdir -p ./data/chainspec/genesis
wget https://github.com/CasperLabs/CasperLabs/releases/download/v0.8.1/system-contracts.tar.gz
tar -xf system-contracts.tar.gz
rm system-contracts.tar.gz
mv mint_install.wasm ./data/chainspec/genesis/
mv pos_install.wasm ./data/chainspec/genesis/
echo "$(cat ./data/validator-id),1000000000000000,1000000" > ./data/chainspec/genesis/accounts.csv
wget https://raw.githubusercontent.com/CasperLabs/CasperLabs/v0.8.1/node/src/main/resources/chainspec/genesis/manifest.toml
mv manifest.toml ./data/chainspec/genesis/

## Create node startup script
echo "DATA_DIR=$(pwd)/data"                       > start_node.sh
echo ""                                          >> start_node.sh
echo "nohup ./casperlabs-engine-grpc-server \\"  >> start_node.sh
echo "       --data-dir \${DATA_DIR} \\"         >> start_node.sh
echo "       \${DATA_DIR}/.casper-node.sock \\"  >> start_node.sh
echo "       > ee.log &"                         >> start_node.sh
echo ""                                          >> start_node.sh
echo "nohup ./node/bin/casperlabs-node run \\"   >> start_node.sh
echo "        -s \\"                             >> start_node.sh
echo "        --server-data-dir \${DATA_DIR} \\" >> start_node.sh
echo "        --server-host 127.0.0.1 \\"        >> start_node.sh
echo "        --server-no-upnp \\"               >> start_node.sh
echo "        --casper-validator-private-key-path \${DATA_DIR}/validator-private.pem \\" >> start_node.sh
echo "        --casper-chain-spec-path \${DATA_DIR}/chainspec \\" >> start_node.sh
echo "        > node.log &"                                       >> start_node.sh
echo ""                                                           >> start_node.sh
echo "echo 'Please wait while node starts...'"                    >> start_node.sh
echo "sleep 1"                                                    >> start_node.sh
echo "tail -f -n0 node.log | grep -qe 'Making the transition to block processing'" >> start_node.sh
echo "if [ $? == 1 ]; then"               >> start_node.sh
echo "    echo 'Node startup failed'"     >> start_node.sh
echo "else"                               >> start_node.sh
echo "    echo 'Node startup successful'" >> start_node.sh
echo "fi"                                 >> start_node.sh
chmod +x start_node.sh

## Create node shutdown script
echo "pkill -f 'io.casperlabs.node.Main run'"    > shutdown_node.sh
echo "pkill -f 'casperlabs-engine-grpc-server'" >> shutdown_node.sh
chmod +x shutdown_node.sh

## Create state cleanup script
echo "rm -rf ./data/blockstorage"  > cleanup.sh
echo "rm -rf ./data/dagstorage"   >> cleanup.sh
echo "rm -rf ./data/global_state" >> cleanup.sh
echo "rm ./data/sqlite.db"        >> cleanup.sh
chmod +x cleanup.sh

## Create deploy script
echo "./client/bin/casperlabs-client \\"                                     > deploy.sh
echo "    --host 127.0.0.1 \\"                                              >> deploy.sh
echo "    deploy \\"                                                        >> deploy.sh
echo "    --from $(cat ./data/validator-id-hex) \\"                         >> deploy.sh
echo "    --session \$1 \\"                                                 >> deploy.sh
echo "    --payment-amount 1 \\"                                            >> deploy.sh
echo "    --private-key ./data/validator-private.pem | \\"                  >> deploy.sh
echo "    awk '{for(i=1;i<=NF;i++) if (\$i==\"Deploy\") print \$(i+1)}' \\" >> deploy.sh
echo "    > deploy_hash"                                                    >> deploy.sh
echo "./client/bin/casperlabs-client --host 127.0.0.1 propose | \\"         >> deploy.sh
echo "    awk '{for(i=1;i<=NF;i++) if (\$i==\"Block\") print \$(i+1)}' \\"  >> deploy.sh
echo "    > block_hash"                                                     >> deploy.sh
echo "echo Deploy \$(cat deploy_hash) included in block \$(cat block_hash)" >> deploy.sh
chmod +x deploy.sh

## Return to main directory
echo "Development environment successfully installed"
cd ..