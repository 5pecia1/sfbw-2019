# San Francisco Blockchain Week 2019 CasperLabs Workshop

The purpose of this repository is to provide the materials needed for the
participants in the SFBW 2019 workshop "Writing a smart contract in Rust for use
with CasperLabs". This repository is not actively maintained, nor has any
guarantee to work after October 31, 2019. If you are reading this after that
date and are interested in CasperLabs, we recommend you check out our [main
repository](https://github.com/CasperLabs/CasperLabs).

If you are a workshop participant, welcome! Please read on to find instructions
for the workshop.

## Hello, CasperLabs

In this section we will set up a development environment, then compile and run a
simple contract.

First, run the install script.

```shell
./install.sh
```

This may take a few minutes as it downloads the CasperLabs node 0.8.1 release
and configures it for use as a sandbox to run contracts. When it completes you
should see a new `dev-env` environment has been created. Next, ensure rust is
installed and compile the `hello-casperlabs` example.

```shell
rustup toolchain install $(cat rust-toolchain)
rustup target add --toolchain $(cat rust-toolchain) wasm32-unknown-unknown

cargo build -p hello-casperlabs --release --target wasm32-unknown-unknown
```

You should now have a new wasm module
`target/wasm32-unknown-unknown/release/hello_casperlabs.wasm`. Now we will use
the node to execute this contract

```shell
cd ./dev-env
./start_node.sh
./deploy.sh ../target/wasm32-unknown-unknown/release/hello_casperlabs.wasm
```

We can now see some information about the execution of the contract.

```shell
./client/bin/casperlabs-client --host 127.0.0.1 show-deploy $(cat deploy_hash)
```

We can also inspect the effect the execution had on the blockchain state. In
the code we have

```rust
let greeting = String::from("Hello, CasperLabs");
let key = storage::new_turef(greeting);
runtime::put_key("hello_casperlabs", &key.into());
```

we can see the string `Hello, CasperLabs` is indeed written under a key with
human-readable name `hello_casperlabs` using a query

```shell
$ ./client/bin/casperlabs-client --host 127.0.0.1 query-state \
    --block-hash $(cat block_hash) \
    --key $(cat ./data/validator-id-hex) \
    --path "hello_casperlabs" \
    --type address

string_value: "Hello, CasperLabs"
```
