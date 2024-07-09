use anchor_lang::prelude::*;

#[account]
pub struct Global {
    pub authority: Pubkey,
    pub pause: bool,
    pub token_vault: Pubkey,    
}


#[account]
pub struct UserData {
    pub owner: Pubkey,
    pub week_count: u32, // 1: week 2: month 3: year
    pub month_count: u32, // 1: week 2: month 3: year
    pub year_count: u32, // 1: week 2: month 3: year
}

#[account]
pub struct UserInfo {
    pub initialized: bool,
    pub index: u32,
    pub owner: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub option: u8, // 1: week 2: month 3: year
    pub status: bool, // false: can stake. true: staking(can't stake) 
}

#[account]
pub struct StakingOption {
    pub week_apy: u8,
    pub month_apy: u8,
    pub year_apy: u8,
}
