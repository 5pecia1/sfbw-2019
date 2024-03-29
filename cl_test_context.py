import requests
import casperlabs_client
import json

from casperlabs_local_net.common import Contract
from casperlabs_local_net.casperlabs_accounts import Account
from casperlabs_local_net.casperlabs_network import CasperLabsNetwork
from casperlabs_client import ABI

GRAPHQL_URL = 'http://localhost:40403/graphql'


class NetworkInstance():
    """ Singleton that holds CasperLabsNetwork object. """
    __network = None
    def __new__(cls, network = None):
        if NetworkInstance.__network is None:
            if network is None:
                raise RuntimeError("NetworkInstance needs to be initialized.")
            else:
                NetworkInstance.__network = network
        elif network is not None:
            raise RuntimeError("NetworkInstance is already initialized.")

        return NetworkInstance.__network


def deploy_and_propose(network: CasperLabsNetwork, contract: str, account: Account) -> (str, str):
    """ Wrapper method for contract deployment. """
    node = network.docker_nodes[0]
    response, deploy_hash = node.p_client.deploy(
        from_address=account.public_key_hex,
        session_contract=contract,
        payment_contract=Contract.STANDARD_PAYMENT,
        gas_price=1,
        public_key=account.public_key_path,
        private_key=account.private_key_path,
        payment_args=[ABI.u512("amount", 10000000)]
    )
    response = node.p_client.propose()
    block_hash = response.block_hash.hex()
    return block_hash, deploy_hash.hex()


def pp_json(json_thing, sort: bool = True, indents: int = 4):
    """ Pretty print JSON object. """
    try:
        if type(json_thing) is str:
            print(json.dumps(json.loads(json_thing), sort_keys=sort, indent=indents))
        else:
            print(json.dumps(json_thing, sort_keys=sort, indent=indents))
    except Exception:
        print(json_thing)


def query(graph_ql_query: str, debug: bool = False) -> dict:
    """ Make a raw GraphQL query to the node. """
    query_json = {"query": graph_ql_query}
    r = requests.post(url=GRAPHQL_URL, json=query_json)
    if debug:
        print(r.text)
    return json.loads(r.text)['data']


def query_global_state(block_hash: str, key_type: str, key: str, path_segments: [str]) -> dict:
    """ Build and execute GraphQL query agains CasperLabs schema. """
    q = f"""
        query {{

          globalState(
            blockHashBase16Prefix: "{block_hash}"
            StateQueries: [
              {{
                keyType: {key_type}
                keyBase16: "{key}"
                # Change single quote to double quote. 
                pathSegments: {str(path_segments).replace("'", '"') }
              }}
            ]
          ) {{
            value {{
              __typename
              ... on IntValue {{
                int: value
              }}
              ... on ByteString {{
                bytes: value
              }}
              ... on IntList {{
                ints: value
              }}
              ... on StringValue {{
                string: value
              }}
              ... on Account {{
                pubKey
                purseId {{
                  uref
                  accessRights
                }}
                namedKeys {{
                  name
                  key {{
                    ...KeyFields
                  }}
                }}
                associatedKeys {{
                  pubKey
                  weight
                }}
                actionThreshold {{
                  deploymentThreshold
                  keyManagementThreshold
                }}
                accountActivity {{
                  keyManagementLastUsed
                  deploymentLastUsed
                  inactivityPeriodLimit
                }}
              }}
              ... on Contract {{
                namedKeys {{
                  name
                  key {{
                    ...KeyFields
                  }}
                }}
              }}
              ... on StringList {{
                strings: value
              }}
              ... on NamedKey {{
                name
                key {{
                  ...KeyFields
                }}
              }}
              ... on BigIntValue {{
                big_int_value: value
                bitWidth
              }}
              ... on Key {{
                ...KeyFields
              }}
              ... on Unit {{
                unit: value
              }}
            }}
          }}
        }}

        fragment KeyFields on Key {{
          value {{
            __typename
            ... on KeyAddress {{
              key_address: value
            }}
            ... on KeyHash {{
              key_hash: value
            }}
            ... on KeyUref {{
              uref
              accessRights
            }}
            ... on KeyLocal {{
              hash
            }}
          }}
        }}
    """
    return query(q, False)['globalState']


def get_latest_block_hash() -> str:
    """ Query node to get latest block hash. """
    q = f"""{{
            dagSlice(depth: 1) {{
                blockHash
            }}
        }}"""
    response = query(q)
    return response['dagSlice'][0]['blockHash']


def next_account(network, init_balance: int = 100000000) -> Account:
    """ Get unused account with initial balance. """
    account: Account = network.get_key()
    node: DockerNode = network.docker_nodes[0]
    block_hash = node.transfer_to_account(account.file_id, init_balance)
    return account


def balance(network: CasperLabsNetwork, address: str, block_hash: str = None) -> int:
    """ Check balance of account. """
    block_hash = block_hash or get_latest_block_hash()
    node = network.docker_nodes[0]
    return node.p_client.balance(address, block_hash)


# Unit tests for NetworkInstance.
if __name__ == "__main__":
    import unittest
    tc = unittest.TestCase()

    # First use should take network as argument
    with tc.assertRaises(RuntimeError):
        nc = NetworkInstance()

    # Initialization works fine.
    my_network = "Hello World"
    NetworkInstance(my_network)

    # Can't initialize it again.
    second_object = "Long time ago..."
    with tc.assertRaises(RuntimeError):
        nc = NetworkInstance(second_object)

    # Nothing new is created.
    tc.assertEqual(NetworkInstance(), my_network)
    tc.assertEqual(NetworkInstance(), my_network)
