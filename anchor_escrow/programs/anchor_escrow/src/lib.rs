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
}
