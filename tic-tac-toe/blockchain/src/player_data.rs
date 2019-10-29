use alloc::string::String;
use alloc::vec::Vec;
use contract_ffi::contract_api::account::PublicKey;
use contract_ffi::contract_api::{storage, Error as ApiError, TURef};
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;
use contract_ffi::uref::AccessRights;
use contract_ffi::value::Value;
use core::convert::{TryFrom, TryInto};
use num_traits::{FromPrimitive, ToPrimitive};
use tic_tac_toe_core::player::Player;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PlayerData {
    piece: Player,
    opponent: PublicKey,
    status_key: TURef<String>,
}

impl PlayerData {
    pub fn read_local(key: PublicKey) -> Option<PlayerData> {
        let value: Option<Value> =
            storage::read_local(key).unwrap_or_revert_with(ApiError::Deserialize);

        value.and_then(|v| v.try_into().ok())
    }

    pub fn write_local(
        key: PublicKey,
        piece: Player,
        opponent: PublicKey,
        status_key: TURef<String>,
    ) {
        let data = PlayerData {
            piece,
            opponent,
            status_key,
        };

        storage::write_local(key, data);
    }

    pub fn piece(&self) -> Player {
        self.piece
    }

    pub fn opponent(&self) -> PublicKey {
        self.opponent
    }

    pub fn status_key(&self) -> TURef<String> {
        self.status_key.clone()
    }
}

impl From<PlayerData> for Value {
    fn from(data: PlayerData) -> Value {
        let mut result = Vec::with_capacity(1 + 32 + 32);

        result.push(data.piece.to_u8().unwrap());
        for byte in data
            .opponent
            .value()
            .iter()
            .chain(data.status_key.addr().iter())
        {
            result.push(*byte);
        }

        Value::ByteArray(result)
    }
}

impl TryFrom<Value> for PlayerData {
    type Error = ();

    fn try_from(value: Value) -> Result<PlayerData, ()> {
        match value {
            Value::ByteArray(mut bytes) => {
                let mut drain = bytes.drain(..);

                let byte = drain.next().ok_or(())?;
                let piece = FromPrimitive::from_u8(byte).ok_or(())?;

                let mut opponent_key = [0u8; 32];
                for i in opponent_key.iter_mut() {
                    let byte = drain.next().ok_or(())?;
                    *i = byte;
                }
                let opponent = PublicKey::new(opponent_key);

                let mut status_key = [0u8; 32];
                for i in status_key.iter_mut() {
                    let byte = drain.next().ok_or(())?;
                    *i = byte;
                }
                let status_key = TURef::new(status_key, AccessRights::READ_ADD_WRITE);

                Ok(PlayerData {
                    piece,
                    opponent,
                    status_key,
                })
            }

            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlayerData;
    use contract_ffi::contract_api::account::PublicKey;
    use contract_ffi::contract_api::TURef;
    use contract_ffi::uref::AccessRights;
    use contract_ffi::value::Value;
    use core::convert::TryInto;
    use tic_tac_toe_core::player::Player;

    #[test]
    fn player_data_round_trip() {
        let player_data = PlayerData {
            piece: Player::X,
            opponent: PublicKey::new([3u8; 32]),
            status_key: TURef::new([5u8; 32], AccessRights::READ_ADD_WRITE),
        };

        let value: Value = player_data.clone().into();

        let player_data_2: PlayerData = value.try_into().expect("Should deserialize");

        assert_eq!(player_data, player_data_2);
    }
}
