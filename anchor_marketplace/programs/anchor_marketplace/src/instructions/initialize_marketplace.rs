use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{error::MarketplaceError, Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account( 
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_bytes()],
        bump,
        space = Marketplace::DISCRIMINATOR.len() + Marketplace::INIT_SPACE
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = marketplace,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitializeMarketplace<'info> {
    pub fn handle(&mut self, name: String, fee_bps: u16, bumps: &InitializeMarketplaceBumps) -> Result<()>{

        require!(name.len() < 4 + 32, MarketplaceError::NameTooLong);
        require!(name.len() > 0, MarketplaceError::UndefinedName);
        self.marketplace.set_inner(Marketplace { admin: self.admin.key(), treasury_bump: bumps.treasury, rewards_bump: bumps.reward_mint, bump: bumps.marketplace, fee_bps, name });

        Ok(())
    }
}
