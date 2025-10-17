use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Invalid Admin")]
    InvalidAdmin,
    #[msg("Unverified Creator can't create token or mint token")]
    InvalidCreator,
}
