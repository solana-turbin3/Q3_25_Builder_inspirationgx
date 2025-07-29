use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::{commit, delegate, ephemeral};
use ephemeral_rollups_sdk::cpi::DelegateConfig;
use ephemeral_rollups_sdk::ephem::{commit_accounts, commit_and_undelegate_accounts};

declare_id!("6MmY6WpTFbonokS9BUKdoTNEtUsLjthU1UDaRQAAeWBv");
const COUNTER_SEEDS: &[u8; 5] = b"COUNT";

#[ephemeral]
#[program]
pub mod magicblock_counter {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCounter>) -> Result<()> {
        ctx.accounts.counter.set_inner(Counter {
            count: 0,
            bump: ctx.bumps.counter,
        });
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;

        if counter.count > 10_000 {
            counter.count = 0;
        }

        Ok(())
    }

    pub fn delegate(ctx: Context<Delegate>) -> Result<()> {
        ctx.accounts.delegate_counter(
            &ctx.accounts.user,
            &[COUNTER_SEEDS, ctx.accounts.user.key().as_ref()],
            DelegateConfig::default(),
        )?;

        Ok(())
    }

    pub fn commit(ctx: Context<Commit>) -> Result<()> {
        commit_accounts(
            &ctx.accounts.user,
            vec![&ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;
        Ok(())
    }

    pub fn increment_and_commit(ctx: Context<IncrementAndCommit>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        counter.count += 1;

        if counter.count > 10_000 {
            counter.count = 0;
        }

        commit_accounts(
            &ctx.accounts.user,
            vec![&ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;
        Ok(())
    }

    pub fn undelegate(ctx: Context<IncrementAndCommit>) -> Result<()> {
        commit_and_undelegate_accounts(
            &ctx.accounts.user,
            vec![&ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;
        Ok(())
    }

    pub fn increment_and_undelegate(ctx: Context<IncrementAndCommit>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        counter.count += 1;

        if counter.count > 10_000 {
            counter.count = 0;
        }

        counter.exit(&ID)?;
        commit_and_undelegate_accounts(
            &ctx.accounts.user,
            vec![&ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [COUNTER_SEEDS, user.key().as_ref()],
        bump,
        space = Counter::DISCRIMINATOR.len() + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [COUNTER_SEEDS, user.key().as_ref()],
        bump = counter.bump,
    )]
    pub counter: Account<'info, Counter>,
}

#[delegate]
#[derive(Accounts)]
pub struct Delegate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [COUNTER_SEEDS, user.key().as_ref()],
        bump = counter.bump,
        del
    )]
    pub counter: Account<'info, Counter>,
}

#[commit]
#[derive(Accounts)]
pub struct Commit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [COUNTER_SEEDS, user.key().as_ref()],
        bump = counter.bump,
    )]
    pub counter: Account<'info, Counter>,
}

#[commit]
#[derive(Accounts)]
pub struct IncrementAndCommit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [COUNTER_SEEDS, user.key().as_ref()],
        bump = counter.bump,
    )]
    pub counter: Account<'info, Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u32,
    pub bump: u8,
}
