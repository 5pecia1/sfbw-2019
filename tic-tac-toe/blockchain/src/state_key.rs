use alloc::vec::Vec;
use contract_ffi::bytesrepr::{self, ToBytes};
use contract_ffi::contract_api::account::PublicKey;

pub struct StateKey([u8; 64]);

impl StateKey {
    pub fn new(x_player: PublicKey, o_player: PublicKey) -> StateKey {
        let mut result = [0u8; 64];

        for (i, j) in x_player
            .value()
            .iter()
            .chain(o_player.value().iter())
            .enumerate()
        {
            result[i] = *j;
        }

        StateKey(result)
    }
}

impl ToBytes for StateKey {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut result = Vec::with_capacity(64);

        for byte in self.0.iter() {
            result.push(*byte);
        }

        Ok(result)
    }
}
