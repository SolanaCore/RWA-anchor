use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::state::Creator;
//discriminator = 4
#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        mint::decimals = 6,
        mint::authority = signer.key(),
        mint::freeze_authority = None,
        mint::token_program = token_program,
        seeds = [b"token_mint", creator.key().as_ref()],
        bump,
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    /// CHECK:  we do read or write from this account
    pub token_metadata: UncheckedAccount<'info>,

    #[account(mut, has_one = signer @ ErrorCode::InvalidCreator)]
    pub creator: Account<'info, Creator>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}


#[event(discriminator = 4)]
pub struct CreateTokenEvent {
    pub token_mint: Pubkey,
    pub token_metadata: Pubkey,
    pub creator: Pubkey
}
