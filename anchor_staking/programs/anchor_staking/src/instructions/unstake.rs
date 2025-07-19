use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token_interface::{approve, Approve, Mint, TokenAccount, TokenInterface},
};

use crate::{error::StakeProgramError, GlobalState, StakeState, UserState};

#[derive(Accounts)]
pub struct UnStake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub collection_mint: InterfaceAccount<'info, Mint>,

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
            mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    }]
    pub metadata: Account<'info, MetadataAccount>,

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
        init_if_needed,
        payer = user,
        seeds = [b"stake", mint.key().as_ref(), global_state.key().as_ref()],
        bump,
        space = StakeState::DISCRIMINATOR.len() + StakeState::INIT_SPACE
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

        Ok(())
    }
}
