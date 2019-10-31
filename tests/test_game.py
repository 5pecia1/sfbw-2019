import json
import unittest
import time

from casperlabs_local_net.casperlabs_accounts import Account
from casperlabs_local_net.common import Contract
from casperlabs_local_net.casperlabs_network import CasperLabsNetwork

from casperlabs_client import ABI

from cl_test_context import NetworkInstance, query_global_state, next_account, get_latest_block_hash, deploy_and_propose, pp_json

class TestCounter(unittest.TestCase):

    # @unittest.skip("Skiped")
    def test_game_play(self):
        network = NetworkInstance()
        
        # Prepare accounts with positive balance.
        account_x: Account = next_account(network, 100000000)
        account_o: Account = next_account(network, 100000000)

        # Deploy counter smart contract with game and get contract hash.
        block_hash, deploy_hash = deploy_and_propose(network, "tic_tac_toe.wasm", account_x)
        ttt_contract_hash = get_contract_hash(account_x, "tic-tac-toe", block_hash);
        ttt_dir_contract_hash = get_contract_hash(account_x, "tic-tac-toe-direct", block_hash)

        # Start game.
        start_args = build_start_args(ttt_dir_contract_hash, account_x, account_o)
        block_hash, deploy_hash = call_ttt(network, account_x, ttt_contract_hash, start_args)

        def make_move(player, row, col):
            move_args = build_move_args(ttt_dir_contract_hash, player, row, col)
            block_hash, deploy_hash = call_ttt(network, player, ttt_contract_hash, move_args)

        # 1st move
        # x| | 
        # -----
        #  | |
        # -----
        #  | |
        make_move(account_x, 0, 0)

        # 2nd move
        # x|o| 
        # -----
        #  | |
        # -----
        #  | |
        make_move(account_o, 0, 1)

        # 3rd move
        # x|o| 
        # -----
        #  |x|
        # -----
        #  | |
        make_move(account_x, 1, 1)

        # 4th move
        # x|o| 
        # -----
        #  |x|o
        # -----
        #  | |
        make_move(account_o, 1, 2)

        # 5th move
        # x|o| 
        # -----
        #  |x|o
        # -----
        #  | |x
        make_move(account_x, 2, 2)

        # Check winner status.
        player_x_outcome = get_player_status(account_x, account_x)
        self.assertEqual(player_x_outcome, f"victorious against {account_o.public_key_hex}")

        # Check defeated status.
        player_o_outcome = get_player_status(account_x, account_o)
        self.assertEqual(player_o_outcome, f"defeated by {account_x.public_key_hex}")


def build_start_args(ttt_dir_contract_hash: str, account_x: Account, account_o: Account) -> str:
    args = '[{"name": "tic-tac-toe-direct-hash", "value": {"bytes_value": "' + ttt_dir_contract_hash + '"}},'
    args += '{"name": "method-name", "value" : {"string_value" : "start"}},'
    args += '{"name": "x_player", "value": {"bytes_value" : "' + account_x.public_key_hex + '"}},' 
    args += '{"name": "o_player", "value": {"bytes_value" : "' + account_o.public_key_hex + '"}}]' 
    return args


def build_move_args(ttt_dir_contract_hash: str, account: Account, row: int, column: int) -> str:
    args = '[{"name": "tic-tac-toe-direct-hash", "value": {"bytes_value": "' + ttt_dir_contract_hash + '"}},'
    args += '{"name": "method-name", "value": {"string_value": "move"}},'
    args += '{"name": "row_position", "value": {"int_value": "' + str(row) + '"}},'
    args += '{"name": "column_position", "value": {"int_value": "' + str(column) + '"}}]'
    return args


def get_contract_hash(account: Account, contract_name: str, block_hash: str = None) -> str:
    block_hash = block_hash or get_latest_block_hash()
    named_keys = query_global_state(
        block_hash,
        "Address",
        account.public_key_hex,
        []
    )[0]['value']['namedKeys']
    for key in named_keys:
        if key['name'] == contract_name:
            return key['key']['value']['key_hash']
    raise Exception(f"Contract '{contract_name}' not found.")


def get_player_status(contract_owner: Account, player: Account, block_hash: str = None):
    block_hash = block_hash or get_latest_block_hash()
    return query_global_state(
        block_hash, 
        "Address", 
        contract_owner.public_key_hex, 
        ["tic-tac-toe-direct", player.public_key_hex]
    )[0]['value']['string']


def call_ttt(network: CasperLabsNetwork, account: Account, ttt_contract_hash: str, json_args: str) -> (str, str):
    """ Wrapper method for contract deployment. """
    node = network.docker_nodes[0]
    payment = node.p_client.node.resources_folder / Contract.STANDARD_PAYMENT
    client = node.p_client.client
    status, deploy_hash = client.deploy(
        from_addr=account.public_key_binary,
        gas_price=1,
        private_key=account.private_key_path,
        public_key=account.public_key_path,
        payment=payment,
        payment_args=[ABI.u512("amount", 10000000)],
        session_hash=bytes.fromhex(ttt_contract_hash),
        session_args=ABI.args_from_json(json_args)
    )
    response = client.propose()
    block_hash = response.block_hash.hex()
    return block_hash, deploy_hash.hex()
