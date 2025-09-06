use anchor_lang::prelude::*;

use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;

use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};
declare_id!("2DFk2oyDSY6dh8cCmnQQ7iayybUuotLwLetoCHqE5dot");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }


    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount, &ctx.bumps)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit_spl(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_spl(amount)?;
        Ok(())
    }

    pub fn withdraw_spl(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_spl(amount, &ctx.bumps)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init, 
        payer = payer,
        space = VaultState::DISCRIMINATOR.len() + VaultState::INIT_SPACE,
        seeds = [b"vault_state", payer.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds=[b"vault"],
        bump,
        owner = system_program.key()
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // Account for Holding SOL
    #[account(
        mut,
        seeds=[b"vault"],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = VaultState::DISCRIMINATOR.len() + VaultState::INIT_SPACE,
        seeds = [b"vault_state", payer.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    pub token_mint: Option<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = payer,
        constraint = user_ata.mint == token_mint.as_ref().expect("Provide token mint").key() @ProgramError::InvalidMint,
        associated_token::mint = token_mint,
        associated_token::authority = payer
    )]
    pub user_ata: Option<InterfaceAccount<'info, TokenAccount>>,

     // Account for Holding Spl Tokens
    #[account(
        init_if_needed,
        payer = payer,
        constraint = vault_ata.mint == token_mint.as_ref().expect("Provide token mint").key() @ProgramError::InvalidMint,
        associated_token::mint = token_mint,
        associated_token::authority = vault_state
    )]
    pub vault_ata: Option<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Payment<'info> {
    fn deposit(&mut self, amount: u64, bumps: &PaymentBumps) -> Result<()> {
        assert!(
            self.payer.to_account_info().lamports() > amount,
            "Insufficient Funds to transfer"
        );
        let accounts = Transfer {
            to: self.vault.to_account_info(),
            from: self.payer.to_account_info(),
        };

        // action if vault does not already exists
        if !self.vault_state.is_initialized {
            self.vault_state.set_inner(VaultState { owner: self.payer.key(), is_initialized: true, vault_state_bump: bumps.vault_state, vault_bump: bumps.vault });
        }

        let cpi = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(cpi, amount)?;

        Ok(())
    }

    fn deposit_spl(&mut self, amount: u64) -> Result<()> {

        let token_program = self.token_program.to_account_info();
        let sender_ata = self.user_ata.as_ref().expect("provide a sender token account").to_account_info();
        let vault_ata = self.vault_ata.as_ref().expect("provide the vault token account").to_account_info();
        let token_mint = self.token_mint.as_ref().expect("You should provide token mint").to_account_info();

        require!(self.user_ata.as_ref().map(|e| e.amount) >= Some(amount), ProgramError::InsufficientBalance);

        let deposit_accounts = TransferChecked {
            authority: self.payer.to_account_info(),
            from: sender_ata,
            to: vault_ata,
            mint: token_mint
        };

        let transfer_cpi = CpiContext::new(token_program, deposit_accounts);

        transfer_checked(transfer_cpi, amount, self.token_mint.as_ref().expect("You should provide token mint").decimals)?;

        Ok(())
    }

    fn withdraw(&mut self, amount: u64, bumps: &PaymentBumps) -> Result<()> {
        assert!(
            self.vault.to_account_info().lamports() > amount,
            "Insufficient Funds to transfe from Vault"
        );

        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", &[bumps.vault]]];

        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.payer.to_account_info(),
        };
        let cpi = CpiContext::new(self.system_program.to_account_info(), accounts)
            .with_signer(signer_seeds);

        transfer(cpi, amount)?;
        Ok(())
    }

    fn withdraw_spl(&mut self, amount: u64, bumps: &PaymentBumps) -> Result<()> {

        let token_program = self.token_program.to_account_info();
        let owner_ata = self.user_ata.as_ref().expect("provide a sender token account").to_account_info();
        let vault_ata = self.vault_ata.as_ref().expect("provide the vault token account").to_account_info();
        let token_mint = self.token_mint.as_ref().expect("You should provide token mint").to_account_info();

        let wallet_balance = self.vault_ata.as_ref().map(|e| e.amount);


        require!(wallet_balance.unwrap() >= amount, ProgramError::InsufficientBalance);

        let deposit_accounts = TransferChecked {
            authority: self.vault_state.to_account_info(),
            from: vault_ata,
            to: owner_ata,
            mint: token_mint
        };

        let payer = self.payer.key();

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault_state",
            payer.as_ref(),
            &[bumps.vault_state]
        ]];

        let transfer_cpi = CpiContext::new_with_signer(token_program, deposit_accounts, signer_seeds);

        transfer_checked(transfer_cpi, amount, self.token_mint.as_ref().expect("You should provide token mint").decimals)?;

        Ok(())
    }
}

impl<'info> Initialize<'info> {
    fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.owner = self.payer.key();
        self.vault_state.is_initialized = true;
        self.vault_state.vault_state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;

        Ok(())
    }
}

#[derive(InitSpace)]
#[account]
pub struct VaultState {
    pub owner: Pubkey,
    pub is_initialized: bool,
    pub vault_state_bump: u8,
    pub vault_bump: u8,
}

#[error_code]
pub enum ProgramError {
    #[msg("You provided wrong token mint for this token")]
    InvalidMint,
    #[msg("You tried to withdraw more than available in balance")]
    InsufficientBalance,
}
