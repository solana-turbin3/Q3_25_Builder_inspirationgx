use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

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

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"rewards", global_state.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = global_state

    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitializeGlobalState<'info> {
    pub fn handle_init(
        &mut self,
        max_stake: u8,
        points_per_stake: u8,
        freeze_period: u32,
        bumps: &InitializeGlobalStateBumps,
    ) -> Result<()> {
        self.global_state.set_inner(GlobalState {
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bumps.reward_mint,
            global_bump: bumps.global_state,
        });
        Ok(())
    }
}
