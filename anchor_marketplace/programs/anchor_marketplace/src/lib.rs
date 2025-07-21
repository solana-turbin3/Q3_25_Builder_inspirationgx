pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CfcoUQwn2VQvk3hvvcbihNG76qXNitLESStbkyfyCDZE");

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<InitializeMarketplace>, params: InitializeParams) -> Result<()> {
        ctx.accounts
            .handle(params.name, params.fee_bps, &ctx.bumps)?;

        Ok(())
    }

    pub fn list_nft(ctx: Context<ListNFT>, params: InitializListingeParams) -> Result<()> {
        ctx.accounts.initialize_listing(params.price, &ctx.bumps)?;
        ctx.accounts.list_nft()?;
        Ok(())
    }

    pub fn purchase_nft(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.make_payment()?;
        ctx.accounts.transfer_nft()?;
        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq)]
pub struct InitializeParams {
    name: String,
    fee_bps: u16,
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq)]
pub struct InitializListingeParams {
    price: u64,
}
