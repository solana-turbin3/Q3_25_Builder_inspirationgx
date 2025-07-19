use anchor_lang::prelude::*;

#[error_code]
pub enum StakeProgramError {
    #[msg("You've hit max stake limit!")]
    MaxStakeReached,
    #[msg("Confirm that you've staked enough to unstake")]
    InsufficientPreviousStakes,
    #[msg("Required time to unfreeze has not passed")]
    UnFreezeTimeNotSatisfied,
}
