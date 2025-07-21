use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub admin: Pubkey,
    pub treasury_bump: u8,
    pub rewards_bump: u8,
    pub bump: u8,
    pub fee_bps: u16,
    #[max_len(32)]
    pub name: String,
}
