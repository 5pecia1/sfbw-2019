use contract_ffi::contract_api::Error as ApiError;

#[repr(u16)]
pub enum Error {
    AlreadyPlaying = 0,
    NoGameFoundForPlayer = 1,
    GameError = 2,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
