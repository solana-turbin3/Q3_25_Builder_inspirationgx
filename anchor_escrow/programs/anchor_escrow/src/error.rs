use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Wrong Maker Account Provider")]
    ProvidedWrongMaker,
}
