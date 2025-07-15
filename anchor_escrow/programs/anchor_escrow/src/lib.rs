use anchor_lang::prelude::*;
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("95yy32pfxhUVFScseYUNiWoDMc121PkUaQYiKVAeNyE8");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        make_offer::handler(ctx, id, token_a_offered_amount, token_b_wanted_amount);
        Ok(())
    }
}
