#![allow(unexpected_cfgs, deprecated)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8KFYtL4PCw2Vv8J5GjwEijpucJSf1iV4wNViSS6o9S7M");

#[program]
pub mod anchor_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_global_state::handler(ctx)
    }
}
