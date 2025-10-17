use anchor_lang::prelude::*;

#[account(discriminator = 2)]
#[derive(InitSpace)]
pub struct Creator {
    pub name: [u8; 32],      
    pub email_id: [u8; 32],  
    pub wallet_pubkey: Pubkey,      
    pub verified: bool,     
    pub bump:u8,
}