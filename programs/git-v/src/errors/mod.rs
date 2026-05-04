use anchor_lang::prelude::*;

#[error_code]
pub enum GitVErrorCode {
    #[msg("Too early to withdraw from escrow")]
    TooEarlyToWithdraw,
}
