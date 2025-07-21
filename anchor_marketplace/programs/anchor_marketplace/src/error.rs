use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Name cannot be undefined")]
    UndefinedName,
    #[msg("Name cannot be more than 32 characters long")]
    NameTooLong,
    #[msg("Error  occured performing arithmetic probable overflow")]
    MathOverflowError,
}
