use anchor_lang::prelude::*;
use crate::state::creator::Creator;
use crate::constants::{CREATOR_SEED, ANCHOR_DISCRIMINATOR};

//discriminator = 2
#[accounts]
pub struct RegisterCreator<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + Creator::INIT_SPACE,
        seeds = [CREATOR_SEED.as_bytes(), wallet_pubkey.key().as_ref()],
        bump,
    )]
    pub creator: Account<'info, Creator>,
    pub system_program: Program<'info, System>,
}


#[event(discriminator = 2)]
pub struct RegisterCreatorEvent {
    pub creator_pda: Pubkey
}