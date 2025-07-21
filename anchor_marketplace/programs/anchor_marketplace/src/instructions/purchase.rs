use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::error::MarketplaceError;
use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub seller: SystemAccount<'info>,

    pub mint: InterfaceAccount<'info, Mint>,
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program
    )]
    pub buyer_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller,
        associated_token::token_program = token_program
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program
    )]
    pub listing_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"listing", marketplace.key().as_ref(), mint.key().as_ref()],
        bump = listing.bump,
        close = seller
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
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

    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Purchase<'info> {
    pub fn make_payment(&mut self) -> Result<()> {
        let token_price = self.listing.price;
        let marketplace_fee = self.marketplace.fee_bps as u64;

        // calculate fee to transfer to marketplace treasury
        let amount_to_transfer_as_fee = token_price
            .checked_mul(marketplace_fee)
            .and_then(|mul_result| mul_result.checked_div(10_000))
            .ok_or_else(|| error!(MarketplaceError::MathOverflowError));

        // calculate remaining amount to transfer to seller
        let amount_to_transfer_to_seller =
            token_price.checked_sub(*amount_to_transfer_as_fee.as_ref().unwrap());

        // building transfer instructions
        let cpi_account_fee_ix = Transfer {
            from: self.buyer.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_account_amount_seller_ix = Transfer {
            from: self.buyer.to_account_info(),
            to: self.seller.to_account_info(),
        };

        transfer(
            CpiContext::new(self.system_program.to_account_info(), cpi_account_fee_ix),
            amount_to_transfer_as_fee.unwrap(),
        )?;
        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                cpi_account_amount_seller_ix,
            ),
            amount_to_transfer_to_seller.unwrap(),
        )?;

        Ok(())
    }

    pub fn transfer_nft(&mut self) -> Result<()> {
        let cpi_accounts_to_transfer_nft = TransferChecked {
            authority: self.listing.to_account_info(),
            from: self.listing_ata.to_account_info(),
            to: self.buyer_ata.to_account_info(),
            mint: self.mint.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"listing",
            &self.marketplace.key().to_bytes(),
            &self.mint.key().to_bytes(),
            &[self.listing.bump],
        ]];

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts_to_transfer_nft,
            signer_seeds,
        );

        transfer_checked(cpi_context, 1, self.mint.decimals)?;

        let cpi_close_accounts = CloseAccount {
            account: self.listing_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            destination: self.buyer.to_account_info(),
        };

        let cpi_close_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_close_accounts,
            signer_seeds,
        );
        close_account(cpi_close_context)?;

        Ok(())
    }
}
