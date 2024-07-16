use anchor_lang::prelude::*;

use crate::{
    constants::{ GLOBAL_STATE_SEED, TOKEN_VAULT_SEED },
    state::Global,
    error::*,
};

use anchor_spl::token::{Mint,Token,TokenAccount};

use std::mem::size_of;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let accts = ctx.accounts;

    accts.global.pause = false;
    accts.global.authority = accts.owner.key();

    Ok(())
}

pub fn set_pause(ctx: Context<ManageGlobal>) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.global.authority.key() == accts.owner.key(), GrimeCode::NotOwner);

    accts.global.pause = true;

    Ok(())
}

pub fn set_start(ctx: Context<ManageGlobal>) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.global.authority.key() == accts.owner.key(), GrimeCode::NotOwner);

    accts.global.pause = false;

    Ok(())
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        space = 8 + size_of::<Global>()
    )]
    pub global: Account<'info, Global>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = owner,
        seeds = [TOKEN_VAULT_SEED],
        bump,
        token::mint = mint,
        token::authority = global
    )]
    pub token_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>, 
}

#[derive(Accounts)]
pub struct ManageGlobal<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global: Account<'info, Global>,
}