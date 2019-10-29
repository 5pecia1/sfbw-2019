#![no_std]

#[macro_use]
extern crate alloc;
extern crate contract_ffi;
extern crate num_traits;

use alloc::vec::Vec;
use contract_ffi::bytesrepr::FromBytes;
use contract_ffi::contract_api::account::PublicKey;
use contract_ffi::contract_api::{runtime, storage, Error as ApiError};
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;

use contract_ffi::key;
use tic_tac_toe_core::game_move::{Move, MoveOutcome};
use tic_tac_toe_core::game_state::GameState;
use tic_tac_toe_core::player::Player;

mod api;
mod error;
mod game_state;
mod player_data;
mod state_key;

use api::Api;
use error::Error;
use player_data::PlayerData;

fn start_game(x_player: PublicKey, o_player: PublicKey) -> Result<(), Error> {
    if PlayerData::read_local(x_player).is_some() {
        return Err(Error::AlreadyPlaying);
    }

    if PlayerData::read_local(x_player).is_some() {
        return Err(Error::AlreadyPlaying);
    }

    let x_str = key::addr_to_hex(&x_player.value());
    let o_str = key::addr_to_hex(&o_player.value());
    let x_status_key = storage::new_turef(format!("playing against {}", o_str));
    let o_status_key = storage::new_turef(format!("playing against {}", x_str));

    game_state::write_local(x_player, o_player, &GameState::new());
    PlayerData::write_local(x_player, Player::X, o_player, x_status_key.clone());
    PlayerData::write_local(o_player, Player::O, x_player, o_status_key.clone());

    runtime::put_key(x_str.as_str(), &x_status_key.into());
    runtime::put_key(o_str.as_str(), &o_status_key.into());

    Ok(())
}

fn take_turn(player: PublicKey, row_position: usize, column_position: usize) -> Result<(), Error> {
    let player_data = PlayerData::read_local(player).ok_or(Error::NoGameFoundForPlayer)?;

    let (x_player, o_player) = if player_data.piece() == Player::X {
        (player, player_data.opponent())
    } else {
        (player_data.opponent(), player)
    };

    let state = game_state::read_local(x_player, o_player).ok_or(Error::NoGameFoundForPlayer)?;
    let player_move = Move {
        player: player_data.piece(),
        row_position,
        column_position,
    };

    match tic_tac_toe_core::take_turn(&state, player_move) {
        Err(_) => Err(Error::GameError),
        Ok(MoveOutcome::Draw) => {
            complete_game(x_player, o_player, None);
            Ok(())
        }
        Ok(MoveOutcome::Winner(winner)) => {
            complete_game(x_player, o_player, Some(winner));
            Ok(())
        }
        Ok(MoveOutcome::Continue(new_state)) => {
            game_state::write_local(x_player, o_player, &new_state);
            Ok(())
        }
    }
}

fn complete_game(x_player: PublicKey, o_player: PublicKey, winner: Option<Player>) {
    let x_player_data = PlayerData::read_local(x_player).unwrap_or_revert();
    let o_player_data = PlayerData::read_local(o_player).unwrap_or_revert();

    let x_str = key::addr_to_hex(&x_player.value());
    let o_str = key::addr_to_hex(&o_player.value());

    let x_status = match winner {
        None => format!("draw against {}", o_str),
        Some(Player::X) => format!("victorious against {}", o_str),
        Some(Player::O) => format!("defeated by {}", o_str),
    };

    let o_status = match winner {
        None => format!("draw against {}", x_str),
        Some(Player::O) => format!("victorious against {}", x_str),
        Some(Player::X) => format!("defeated by {}", x_str),
    };

    storage::write(x_player_data.status_key(), x_status);
    storage::write(o_player_data.status_key(), o_status);

    storage::write_local(x_player, ());
    storage::write_local(o_player, ());
}

fn concede(player: PublicKey) -> Result<(), Error> {
    let player_data = PlayerData::read_local(player).ok_or(Error::NoGameFoundForPlayer)?;
    let (x_player, o_player) = if player_data.piece() == Player::X {
        (player, player_data.opponent())
    } else {
        (player_data.opponent(), player)
    };
    complete_game(x_player, o_player, Some(player_data.piece().other()));
    Ok(())
}

pub fn get_arg_or_revert<T: FromBytes>(i: u32) -> T {
    runtime::get_arg(i)
        .unwrap_or_revert_with(ApiError::MissingArgument)
        .unwrap_or_revert_with(ApiError::InvalidArgument)
}

#[no_mangle]
pub extern "C" fn dispatch() {
    match Api::from_args(0) {
        Api::Start(x_player, o_player) => start_game(x_player, o_player).unwrap_or_revert(),

        Api::Move(row_position, column_position) => {
            let player = runtime::get_caller();
            take_turn(player, row_position as usize, column_position as usize).unwrap_or_revert();
        }

        Api::Concede => {
            let player = runtime::get_caller();
            concede(player).unwrap_or_revert();
        }
    }
}

#[no_mangle]
pub extern "C" fn indirect() {
    let hash: [u8; 32] = get_arg_or_revert(0);
    let pointer = contract_ffi::contract_api::ContractRef::Hash(hash);
    match Api::from_args(1) {
        Api::Start(x_player, o_player) => {
            runtime::call_contract(pointer, &("start", x_player, o_player), &Vec::new())
        }

        Api::Move(row_position, column_position) => runtime::call_contract(
            pointer,
            &("move", row_position, column_position),
            &Vec::new(),
        ),

        Api::Concede => runtime::call_contract(pointer, &("concede",), &Vec::new()),
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let dispatch_hash = storage::store_function_at_hash("dispatch", Default::default());
    let indirect_hash = storage::store_function_at_hash("indirect", Default::default());

    runtime::put_key("tic-tac-toe-direct", &dispatch_hash.into());
    runtime::put_key("tic-tac-toe", &indirect_hash.into());
}
