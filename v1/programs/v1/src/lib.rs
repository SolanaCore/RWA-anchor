pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;
declare_id!("4B8DqP2mGWscbgfQKcsnZPG6jtYuXs7YFY9Sg4KWi3XV");

pub struct ADMIN {
    declare_id("52nvBaMXujpVYf6zBUvmQtHEZc4kAncRJccXG99F6yrg");
}

#[program]
pub mod v1 {

    //0 - InitGlobalConfig
    //1 - RegisterCreator
    //2 - OnboardCreator
    //3 - CreateToken
    //4 - MintToken
    //5 - Batch Process  
}
