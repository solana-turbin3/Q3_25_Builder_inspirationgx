use crate::state::Offer;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, close_account, CloseAccount}
};


use crate::error::EscrowError::{ProvidedWrongMaker};

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(
        mut,
        mint::token_program = token_program
    )]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        mint::token_program = token_program
    )]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_account_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
        associated_token::mint = token_mint_a
    )]
    pub taker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, constraint = maker.key() == offer.maker.key() @ProvidedWrongMaker)]
    pub maker: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program

    )]
    pub maker_token_account_b: InterfaceAccount<'info, TokenAccount>,


    #[account(
        mut,   
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}


impl<'info> TakeOffer<'info>{
    pub fn handler(&mut self) -> Result<()> {
        // , bumps: &TakeOfferBumps

    // transferring funds to the maker of the offer
    let take_cpi_accounts =   TransferChecked {
        authority: self.taker.to_account_info(),
        from: self.taker_token_account_b.to_account_info(),
        to: self.maker.to_account_info(),
        mint: self.token_mint_b.to_account_info()
        };
    let cpi_context =  CpiContext::new(self.associated_token_program.to_account_info(), take_cpi_accounts);

    transfer_checked(
        cpi_context,
        self.offer.amount_token_b_wanted,
        self.token_mint_b.decimals
    )?;


    // transferring funds to the person accepting the offer
    let transfer_make_cpi_accounts = TransferChecked {
        authority: self.offer.to_account_info(),
        from: self.vault.to_account_info(),
        to: self.taker_token_account_a.to_account_info(),
        mint: self.token_mint_a.to_account_info()
    };

    let signer_seeds: &[&[&[u8]]] = &[&[b"offer"], &[&self.maker.key().to_bytes()], &[&self.offer.bump.to_le_bytes()]];

    let tf_cpi_context = CpiContext::new_with_signer(self.associated_token_program.to_account_info(), transfer_make_cpi_accounts, signer_seeds);

    transfer_checked(
        tf_cpi_context,
        self.vault.amount,
        self.token_mint_a.decimals
    )?;

    // attempting to close the offer account
    close_account(
        CpiContext::new(self.token_program.to_account_info(),CloseAccount {
          account:  self.offer.to_account_info(),
          authority: self.maker.to_account_info(),
        destination: self.taker.to_account_info()
        })
    )?;


        Ok(())
    }
}