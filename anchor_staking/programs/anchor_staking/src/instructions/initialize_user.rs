use anchor_lang::prelude::*;


use crate::{UserState};

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

     #[account(
        init,
        payer = user,
        seeds = [b"user", user.key().as_ref()],
        bump,
        space = UserState::DISCRIMINATOR.len() + UserState::INIT_SPACE
        
    )]
    pub user_account: Account<'info, UserState>,

    pub system_program: Program<'info, System>,
   
}


impl<'info> InitializeUser<'info> {
    pub fn handle_initialize(&mut self, bumps: &InitializeUserBumps) -> Result<()>{
        self.user_account.set_inner(UserState { points: 0, amount_staked: 0, bump: bumps.user_account });
        Ok(())
    }
}