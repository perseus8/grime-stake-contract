use anchor_lang::prelude::*;

#[error_code]
pub enum GrimeCode {
    #[msg("Not Owner")]
    NotOwner,

    #[msg("You are staking token now")]
    AlreadyStake,

    #[msg("It is not time to withdraw, please wait for more time.")]
    NotWithdrawTime,

    #[msg("The option you provide is not valid.")]
    InvalidOption
}
