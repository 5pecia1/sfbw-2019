use crate::state_key::StateKey;
use alloc::vec::Vec;
use contract_ffi::contract_api::account::PublicKey;
use contract_ffi::contract_api::{storage, Error as ApiError};
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;
use contract_ffi::value::Value;
use num_traits::{FromPrimitive, ToPrimitive};
use tic_tac_toe_core::game_state::{CellState, GameState, N_CELLS};

pub fn read_local(x_player: PublicKey, o_player: PublicKey) -> Option<GameState> {
    let state_key = StateKey::new(x_player, o_player);

    let value: Option<Value> =
        storage::read_local(state_key).unwrap_or_revert_with(ApiError::Deserialize);

    value.and_then(from_value)
}

pub fn write_local(x_player: PublicKey, o_player: PublicKey, state: &GameState) {
    let state_key = StateKey::new(x_player, o_player);
    let value = to_value(state).unwrap_or_revert();
    storage::write_local(state_key, value);
}

fn to_value(state: &GameState) -> Option<Value> {
    let player_byte = state.active_player.to_u8()?;
    let mut bytes = Vec::with_capacity(1 + N_CELLS);

    bytes.push(player_byte);
    for cell in state.board.iter() {
        bytes.push(cell.to_u8()?);
    }

    Some(Value::ByteArray(bytes))
}

fn from_value(value: Value) -> Option<GameState> {
    match value {
        Value::ByteArray(mut bytes) => {
            let mut drain = bytes.drain(..);

            let byte = drain.next()?;
            let active_player = FromPrimitive::from_u8(byte)?;
            let mut board = [CellState::Empty; N_CELLS];

            for cell in board.iter_mut() {
                let byte = drain.next()?;
                *cell = FromPrimitive::from_u8(byte)?;
            }

            Some(GameState {
                active_player,
                board,
            })
        }

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use tic_tac_toe_core::game_state::{CellState, GameState, N_CELLS};
    use tic_tac_toe_core::player::Player;

    #[test]
    fn game_state_round_trip() {
        let state = GameState {
            active_player: Player::O,
            board: [CellState::X; N_CELLS],
        };

        let value = super::to_value(&state).expect("Should serialize");

        let state_2 = super::from_value(value).expect("Should deserialize");

        assert_eq!(state, state_2);
    }
}
