use anchor_lang::prelude::*;
use crate::state::global_config::GlobalConfig;
use crate::constants::{ANCHOR_DISCRIMINATOR, GLOBAL_CONFIG_SEED};
use crate::error::ErrorCode;

//discriminator = 0
#[derive(Accounts)]
pub struct InitGlobalConfig<'info> {
    #[account(init_if_needed, payer = admin, space = ANCHOR_DISCRIMINATOR + GlobalConfig::INIT_SPACE, seeds = [GLOBAL_CONFIG_SEED.as_bytes()], bump)]
    pub global_config: Account<'info, GlobalConfig>,
    #[account(mut, address = crate::ADMIN::ID @ ErrorCode::InvalidAdmin)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[event(discriminator = 0)]
pub struct InitGlobalConfigEvent {
    pub global_config: Pubkey,
    pub authorised_signers: Vec<Pubkey>,
    pub admin: Pubkey,
}