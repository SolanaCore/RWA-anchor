use anchor_lang::prelude::*;

#[account(discriminator = 1)]
#[derive(InitSpace)]
pub struct GlobalConfig {
    pub config_authority: Pubkey,
    pub kyc_authority: Pubkey,
    pub active: bool,
    pub open_time: u64,
    pub max_decimal: u8,
    pub fees_bps: u8,   //we'll use this in future for fee deduction on minting
    pub bump: u8,
}
