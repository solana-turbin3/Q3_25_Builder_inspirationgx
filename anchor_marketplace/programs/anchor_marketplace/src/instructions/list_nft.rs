use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct ListNFT<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = seller,
        associated_token::token_program = token_program

    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = seller,
        seeds = [b"listing", marketplace.key().as_ref(), mint.key().as_ref()],
        bump,
        space = Listing::DISCRIMINATOR.len() + Listing::INIT_SPACE
    )]
    pub listing: Account<'info, Listing>,

    #[account{
        init_if_needed,
        payer = seller,
        associated_token::authority = listing,
        associated_token::mint = mint,
        associated_token::token_program = token_program
    }]
    pub listing_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.mint.key() == mint.key(),
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> ListNFT<'info> {
    pub fn initialize_listing(&mut self, price: u64, bumps: &ListNFTBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            seller: self.seller.key(),
            mint: self.mint.key(),
            price,
            bump: bumps.listing,
            is_active: true,
        });
        Ok(())
    }

    pub fn list_nft(&mut self) -> Result<()> {
        let cpi_accounts = TransferChecked {
            authority: self.seller.to_account_info(),
            from: self.seller_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.listing_ata.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer_checked(cpi_ctx, 1, self.mint.decimals)?;
        Ok(())
    }
}
