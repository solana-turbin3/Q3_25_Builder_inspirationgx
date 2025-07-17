use anchor_lang::prelude::*;

use crate::GlobalState;

#[derive(Accounts)]
pub struct InitializeGlobalState<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"global_state"],
        bump,
        space = GlobalState::DISCRIMINATOR.len() + GlobalState::INIT_SPACE
    )]
    pub global_state: Account<'info, GlobalState>,

    pub reward_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
}
