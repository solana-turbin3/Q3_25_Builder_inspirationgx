use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token_interface::{revoke, Mint, Revoke, TokenAccount, TokenInterface},
};

use crate::{error::StakeProgramError, GlobalState, StakeState, UserState};

#[derive(Accounts)]
pub struct UnStake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
        associated_token::authority = user
    )]
    pub user_mint_ata: InterfaceAccount<'info, TokenAccount>,

    #[account{
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    }]
    pub edition: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"global_state"],
        bump = global_state.global_bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_state.bump
    )]
    pub user_state: Account<'info, UserState>,

    #[account(
        mut,
        close = user,
        seeds = [b"stake", mint.key().as_ref(), global_state.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeState>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> UnStake<'info> {
    pub fn unstake_handler(&mut self) -> Result<()> {
        require(
            self.user_state.amount_staked >= 1,
            StakeProgramError::InsufficientPreviousStakes,
        );

        let time_elapsed: u32 =
            ((Clock::get()?.unix_timestamp - self.stake_account.staked_at) / 86400) as u32;

        require!(
            time_elapsed > self.global_state.freeze_period,
            StakeProgramError::UnFreezeTimeNotSatisfied
        );

        self.user_state.points += (self.global_state.points_per_stake as u32) * time_elapsed;

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"stake",
            self.mint.to_account_info().key().as_ref(),
            self.global_state.to_account_info().key().as_ref(),
            &[self.stake_account.bump],
        ]];

        // Unfreezing the delegated token

        ThawDelegatedAccountCpi::new(
            &self.metadata_program.to_account_info(),
            ThawDelegatedAccountCpiAccounts {
                delegate: &self.stake_account.to_account_info(),
                token_account: &self.user_mint_ata.to_account_info(),
                edition: &self.edition.to_account_info(),
                mint: &self.mint.to_account_info(),
                token_program: &self.token_program.to_account_info(),
            },
        )
        .invoke_signed(signer_seeds)?;

        let cpi_revoke_accounts = Revoke {
            authority: self.user.to_account_info(),
            source: self.user_mint_ata.to_account_info(),
        };

        // revoking the previous approve delegate

        revoke(CpiContext::new(
            self.token_program.to_account_info(),
            cpi_revoke_accounts,
        ))?;

        if self.user_state.amount_staked > 0 {
            self.user_state.amount_staked -= 1
        };

        Ok(())
    }
}
