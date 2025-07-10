use anchor_lang::prelude::*;

use anchor_lang::system_program::{transfer, Transfer};
declare_id!("Bw5DfdLdTT6JSzmM3E3YNu7NJ9gGNdRiCmiz4GukhxVY");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, &ctx.bumps)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init, payer = payer,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"vault_state", payer.key().as_ref()],
        bump)]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        init,
        payer= payer,
        seeds=[b"vault"],
        bump,
        space = 0,
        owner = system_program.key()
        )]
    /// CHECK: System Program Account
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut, seeds=[b"vault"], bump)]
    pub vault: SystemAccount<'info>,

    #[account(seeds = [b"vault_state", payer.key().as_ref()], bump)]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    fn deposit(&mut self, amount: u64) -> Result<()> {
        assert!(
            self.payer.to_account_info().lamports() > amount,
            "Insufficient Funds to transfer"
        );
        let accounts = Transfer {
            to: self.vault.to_account_info(),
            from: self.payer.to_account_info(),
        };

        let cpi = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(cpi, amount)?;

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
