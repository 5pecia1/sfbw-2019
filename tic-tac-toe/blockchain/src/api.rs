use alloc::string::String;
use contract_ffi::contract_api::account::PublicKey;
use contract_ffi::contract_api::{runtime, Error as ApiError};

pub enum Api {
    Start(PublicKey, PublicKey),
    Move(u32, u32),
    Concede,
}

impl Api {
    pub fn from_args(start_index: u32) -> Api {
        let method_name: String = super::get_arg_or_revert(start_index);

        match method_name.as_str() {
            "start" => {
                let x_player = super::get_arg_or_revert(start_index + 1);
                let o_player = super::get_arg_or_revert(start_index + 2);
                Api::Start(x_player, o_player)
            }

            "move" => {
                let row_position: u32 = super::get_arg_or_revert(start_index + 1);
                let column_position: u32 = super::get_arg_or_revert(start_index + 2);
                Api::Move(row_position, column_position)
            }

            "concede" => Api::Concede,

            _ => runtime::revert(ApiError::InvalidArgument),
        }
    }
}
