use anchor_lang::prelude::*;

use anchor_spl::token_interface::{
    close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
    TransferChecked,
};

// Transfer tokens from one account to another
// If transferring from a token account owned by a PDA, owning_pda_seeds must be provided.
pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &AccountInfo<'info>,
    token_program: &Interface<'info, TokenInterface>,
    owning_pda_seeds: Option<&[&[u8]]>,
) -> Result<()> {
    let transfer_accounts = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    // Only one signer seed (the PDA that owns the token account) is needed, so we create an array with the seeds
    let signers_seeds = owning_pda_seeds.map(|seeds| [seeds]);

    // Do the transfer, by calling transfer_checked - providing a different CPIU context
    // depending on whether we're sending tokens from a PDA or not
    transfer_checked(
        if let Some(seeds_arr) = signers_seeds.as_ref() {
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                transfer_accounts,
                seeds_arr,
            )
        } else {
            CpiContext::new(token_program.to_account_info(), transfer_accounts)
        },
        *amount,
        mint.decimals,
    )
}

// Close a token account and send the rent to the specified destination
// If the token account is owned by a PDA, owning_pda_seeds must be provided.
pub fn close_token_account<'info>(
    token_account: &InterfaceAccount<'info, TokenAccount>,
    destination: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    token_program: &Interface<'info, TokenInterface>,
    owning_pda_seeds: Option<&[&[u8]]>,
) -> Result<()> {
    let close_accounts = CloseAccount {
        account: token_account.to_account_info(),
        destination: destination.to_account_info(),
        authority: authority.to_account_info(),
    };

    // Only one signer seed (the PDA that owns the token account) is needed, so we create an array with the seeds
    let signers_seeds = owning_pda_seeds.map(|seeds| [seeds]);

    close_account(if let Some(seeds_arr) = signers_seeds.as_ref() {
        CpiContext::new_with_signer(token_program.to_account_info(), close_accounts, seeds_arr)
    } else {
        CpiContext::new(token_program.to_account_info(), close_accounts)
    })
}
