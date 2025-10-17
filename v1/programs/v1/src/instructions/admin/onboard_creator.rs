use anchor_lang::prelude::*;
use crate::constants::{CREATOR_SEED};
use crate::state::creator::Creator;
//discriminator = 3
#[derive(Accounts)]
pub struct OnboardCreator<'info> {
    //any of the pubkey from the array of authorised signer can verify and release attestation
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [CREATOR_SEED.as_bytes(), wallet_pubkey.key().as_ref()],
        bump = creator.bump,
    )]
    pub creator: Account<'info, Creator>,

    /// CHECK:  we don't read or write from this account
    pub wallet_pubkey: UncheckedAccount<'info>,

    /// CHECK: we don't read or write from this account
    pub credential_sas: UncheckedAccount<'info>,


    pub solana_attestation_program: Program<'info, SolanaAttestation>,
    pub system_program: Program<'info, System>,
}

#[event(discriminator = 3)]
pub struct OnboardCreatorEvent {
    pub creator: Pubkey,
    pub credential_sas: Pubkey,
}
