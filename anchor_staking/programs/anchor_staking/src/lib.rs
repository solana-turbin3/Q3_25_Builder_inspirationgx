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

    pub fn initialize(ctx: Context<InitializeGlobalState>, params: InitializeGlobal) -> Result<()> {
        ctx.accounts.handle_init(
            params.max_stake,
            params.points_per_stake,
            params.freeze_period,
            &ctx.bumps,
        )
    }

    pub fn initialize_stake_account(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.handle_initialize(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake_handler(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<UnStake>) -> Result<()> {
        ctx.accounts.unstake_handler()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeGlobal {
    max_stake: u8,
    points_per_stake: u8,
    freeze_period: u32,
}
