use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    pub maker: SystemAccount<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(init_if_needed, payer = taker, associated_token::mint = mint_a, associated_token::authority = taker)]
    pub taker_ata_a: InterfaceAccount<'info, TokenInterface>,

    #[account(mut, associated_token::mint = mint_b, associated_token::authority = taker)]
    pub taker_ata_b: InterfaceAccount<'info, TokenInterface>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenInterface>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'inf, AssociatedToken>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        associated_token::authority = escrow,
        associated_token::mint = mint_a,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
}

impl<'info> Take<'info> {
    pub fn deposit(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.maker_ata_b.to_account_info(),
            to: self.maker_ata_b..to_account_info(),
            mint: self.mint_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_acounts);
        Ok(())
    }
}
