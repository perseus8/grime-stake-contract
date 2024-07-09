use anchor_lang::prelude::*;

use crate::{
    constants::{ GLOBAL_STATE_SEED, TOKEN_VAULT_SEED, USER_INFO_SEED },
    state::{Global, UserInfo, UserData},
    error::*,
};
use std::mem::size_of;
use anchor_spl::token::{Mint,Token,TokenAccount,Transfer, transfer};

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_token_account.to_account_info().clone(),
            to: ctx.accounts.token_vault.to_account_info().clone(),
            authority: ctx.accounts.user.to_account_info().clone(),
        },
    );
    transfer(cpi_ctx, amount)?;

    Ok(())
}

pub fn stake(ctx: Context<StakeGrime>, option: u8, index: u32, amount: u64) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.user_info.status != true, GrimeCode::AlreadyStake);

    if !accts.user_info.initialized {
        accts.user_info.owner = accts.user.key();
        accts.user_info.initialized = true;
    } else {
        require!(accts.user_info.owner ==  accts.user.key(), GrimeCode::NotOwner);
    }

    accts.user_info.start_time = accts.clock.unix_timestamp;
    accts.user_info.amount = amount;
    accts.user_info.option = option;
    accts.user_info.status = true;
    accts.user_info.index = index;

    match option {
        1_u8 => {
            require!(accts.user_data.week_count + 1 == index, GrimeCode::AlreadyStake);
            accts.user_data.week_count += 1;
        },
        2_u8 => {
            require!(accts.user_data.month_count + 1 == index, GrimeCode::AlreadyStake);
            accts.user_data.month_count += 1;
        },
        3_u8 => {
            require!(accts.user_data.year_count + 1 == index, GrimeCode::AlreadyStake);
            accts.user_data.year_count += 1;
        },
        0_u8 | 4_u8..=u8::MAX => todo!() 
    }

    // stake grime tokens to the token vault account
    let cpi_ctx = CpiContext::new(
        accts.token_program.to_account_info(),
        Transfer {
            from: accts.user_token_account.to_account_info().clone(),
            to: accts.token_vault.to_account_info().clone(),
            authority: accts.user.to_account_info().clone(),
        },
    );
    transfer(cpi_ctx, amount)?;
    //  update the user info
    Ok(())
}

pub fn unstake(ctx: Context<StakeGrime>, option: u8, index: u32) -> Result<()> {
    let accts = ctx.accounts;

    require!(accts.user_info.status == true, GrimeCode::AlreadyStake);
    require!(accts.user_info.owner ==  accts.user.key(), GrimeCode::NotOwner);

    let mut passing_time: i64 = 0;
    let mut apy: u64 = 0;

    match accts.user_info.option {
        1_u8 => {
            passing_time = 7 * 3600 * 24;

            apy = 1;
        },
        2_u8=> {
            passing_time = 7 * 3600 * 24 * 30;

            apy = 5;
        },
        3_u8 => {
            passing_time = 7 * 3600 * 24 * 365;

            apy = 69;
        },
        _ => {
            return Err(GrimeCode::InvalidOption.into());
        }
    }

    require!(accts.clock.unix_timestamp > accts.user_info.start_time + passing_time, GrimeCode::NotWithdrawTime);
 
    accts.user_info.start_time = accts.clock.unix_timestamp;
    accts.user_info.option = 0;
    accts.user_info.status = false;

    let claim_amount = accts.user_info.amount;
    msg!("claim_amount{}:",claim_amount);
    let reward = claim_amount  * apy / 100;
    msg!("reward{}:",reward);

    accts.user_info.amount = 0;

    let (_, bump) = Pubkey::find_program_address(&[GLOBAL_STATE_SEED], &ctx.program_id);
    let vault_seeds = &[GLOBAL_STATE_SEED, &[bump]];
    let signer = &[&vault_seeds[..]];


    // stake grime tokens to the token vault account
    let cpi_ctx = CpiContext::new(
        accts.token_program.to_account_info(),
        Transfer {
            from: accts.token_vault.to_account_info().clone(),
            to: accts.user_token_account.to_account_info().clone(),
            authority: accts.global.to_account_info().clone(),
        },
    );
    transfer(
        cpi_ctx.with_signer(signer),
        claim_amount + reward,
    )?;
    //  update the user info
    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user:  Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [TOKEN_VAULT_SEED],
        bump,
        token::mint = mint,
        token::authority = global,
    )]
    pub token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>, 
}

#[derive(Accounts)]
#[instruction(option: u8, index: u32)]
pub struct StakeGrime<'info> {
    #[account(mut)]
    pub user:  Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [USER_INFO_SEED, user.key().as_ref()],
        bump,
        space = 8 + size_of::<UserData>()
    )]
    pub user_data: Box<Account<'info, UserData>>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [USER_INFO_SEED, user.key().as_ref(), option.to_le_bytes().as_ref(),index.to_le_bytes().as_ref()],
        bump,
        space = 8 + size_of::<UserInfo>()
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [TOKEN_VAULT_SEED],
        bump,
        token::mint = mint,
        token::authority = global,
    )]
    pub token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>, 
    pub clock:  Sysvar<'info, Clock>,
}