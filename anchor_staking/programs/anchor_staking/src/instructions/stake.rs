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
pub struct Stake<'info> {
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

impl<'info> Stake<'info> {
    pub fn stake_handler(&mut self, bumps: &StakeBumps) -> Result<()> {
        require!(
            self.user_state.amount_staked < self.global_state.max_stake,
            StakeProgramError::MaxStakeReached
        );

        self.stake_account.set_inner(StakeState {
            owner: self.user.key(),
            mint: self.mint.key(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: bumps.stake_account,
        });

        let cpi_program = self.token_program.to_account_info();

        let cpi_account = Approve {
            authority: self.user.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            to: self.user_mint_ata.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_account);
        approve(cpi_context, 1)?; // approving delegate to be able to spend 1 token, since it's an nft and supply is 1

        let mint = self.mint.key();
        let global_state = self.global_state.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"stake",
            mint.as_ref(),
            global_state.as_ref(),
            &[self.stake_account.bump],
        ]];

        FreezeDelegatedAccountCpi::new(
            &self.metadata_program.to_account_info(),
            FreezeDelegatedAccountCpiAccounts {
                delegate: &self.stake_account.to_account_info(),
                edition: &self.edition.to_account_info(),
                mint: &self.mint.to_account_info(),
                token_account: &self.user_mint_ata.to_account_info(),
                token_program: &self.token_program.to_account_info(),
            },
        )
        .invoke_signed(signer_seeds)?;

        self.user_state.amount_staked += 1;

        Ok(())
    }
}
