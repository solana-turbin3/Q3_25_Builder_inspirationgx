use anchor_lang::prelude::*;

declare_id!("DAvppGYLdn7Ux17JtXRDkSddx91VQMWWW4UvGKG4x51S");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
