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

## Tic-Tac-Toe Example

Let's try a more complex example. In `tic-tac-toe` is a simple tic-tac-toe (Xs
and Os) game which shows some of the advantages of being able to develop in Rust
(a general purpose systems programming language). `tic-tac-toe/core` contains
platform agnostic logic for the game, the logic in this module is used in the
other two modules: `cli` and `blockchain`. `tic-tac-toe/cli` is a simple command
line app which lets two users sitting at the same computer to play against each
other, while `tic-tac-toe/blockchain` presents the same core logic as a smart
contract on the blockchain instead.

You will also notice that some Rust unit tests have been written, showing how
individual parts of the code can be tested independent of any complex blockchain
environment. From the main directory (you may need to `cd ..` from `dev-env` if
you are still there from the previous section) run

```shell
cargo test
```

Let's try compiling and running the blockchain version of the app.

```shell
cargo build -p tic-tac-toe-blockchain --release --target wasm32-unknown-unknown
```

Move into the development environment again,

```shell
cd ./dev-env
```

If the node is still running from the previous section then you can proceed to
running the shell scripts provided with the tic-tac-toe app, otherwise you will
need to run `./start_node.sh` again.

```shell
../tic-tac-toe/blockchain/scripts/store-contract.sh
../tic-tac-toe/blockchain/scripts/start-game.sh
```

Now we are ready to play! You can use use the `x-move` and `o-move` scripts to
place pieces. Each takes two arguments, representing the row and column you wish
to place, where `0 0` is the top left corner of the grid. For example, a game in
which O is not very clever may proceed as

```shell
../tic-tac-toe/blockchain/scripts/x-move.sh 0 0
../tic-tac-toe/blockchain/scripts/o-move.sh 0 1
../tic-tac-toe/blockchain/scripts/x-move.sh 1 0
../tic-tac-toe/blockchain/scripts/o-move.sh 1 1
../tic-tac-toe/blockchain/scripts/x-move.sh 2 0 ## x wins!
```

You can see the result of the game using the `player-status` script

```shell
$ ../tic-tac-toe/blockchain/scripts/player-status.sh $(cat ./data/validator-id-hex)
string_value: "victorious against a28c5afecef083a18f10a9013748c99b1a3dc2d40e34551936ddb93fb841fd99"

$ ../tic-tac-toe/blockchain/scripts/player-status.sh $(cat ./data/account-id-hex)
string_value: "defeated by 9e86d542ab6604178fbeaf4fd4a35d7356156c86dac8fd78c6f95c20beceac63"
```

Try playing around with it yourself. Note that you do not need to call
`store-contract.sh` anymore, the contract is already stored on chain, so if you
wish to play again you simply call `start-game.sh` again. Remember that you can
always check on the the status of a transaction after running one of the scripts
with 

```shell
./client/bin/casperlabs-client --host 127.0.0.1 show-deploy $(cat deploy_hash)
```

In particular, you can see error codes returned, if any (you will see some if
you try to do invalid actions).

It is left as an exercise to the reader to write a nice UI over the blockchain
app so that you don't have to keep track of the board state in your head. You
may wish to run the console app version at the same time, as this at least has
some rudimentary visualization. To compile and run the console app, return to
the main directory (`cd ..`) and 

```shell
cargo build -p tic-tac-toe-cli --release
./target/release/tic_tac_toe
```
To play the console app, you again input the row and column for your move, but
here they are separated by a comma (no space), e.g. `0,0`.

## Programmatic Testing with Python

Based on Maciej's
[smart-contract-template](https://github.com/zie1ony/casperlabs-smart-contract-template)
repo.

Prerequisites:

```shell
sudo apt install protobuf-compiler git docker docker-compose python3-pip
sudo pip3 install pipenv docker
```

Set up environment:

```shell
make prepare
make console
```

Run tests:

```python
>>> run_tests()
```

Exit when finished:

```python
>>> quit()
```

The test is defined in `tests/test_game.py`. It essentially performs the same
logic we just did manually using the scala client and shell scripts, but now it
is done programmatically via the python client.
