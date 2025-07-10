use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
struct Offer {
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub id: u64,
    pub amount_token_b_wanted: u64,
    pub bump: u8,
    pub maker: Pubkey,
}
