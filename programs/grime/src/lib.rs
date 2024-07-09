pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("6Ljyy7jHPEZ6TqBDteGTZMEnZJNrzRkGHnEtktm8mp8c");

#[program]
pub mod grime {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn set_pause(ctx: Context<ManageGlobal>) -> Result<()> {
        instructions::set_pause(ctx)
    }

    pub fn set_start(ctx: Context<ManageGlobal>) -> Result<()> {
        instructions::set_start(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn stake(ctx: Context<StakeGrime>,option: u8, index: u32,amount: u64) -> Result<()> {
        instructions::stake(ctx, option, index,amount)
    }

    pub fn unstake(ctx: Context<StakeGrime>, option: u8, index: u32) -> Result<()> {
        instructions::unstake(ctx, option, index)
    }
}
