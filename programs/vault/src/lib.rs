use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("EvbvvTWJHMtEQdSyADeJLbTKMpzK2zHsw1Mhmw7sDaVc");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.maker = *ctx.accounts.maker.key;
        msg!("Engineer Xage Vault Initialized!");
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.maker.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        );
        transfer(cpi_context, amount)?;
        msg!(" I have deposited {} lamports", amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let seeds = &[
            b"vault",
            ctx.accounts.state.to_account_info().key.as_ref(),
            &[ctx.bumps.vault],
        ];
        let signer = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.maker.to_account_info(),
            },
            signer,
        );
        transfer(cpi_context, amount)?;
        msg!("I have withdrawn {} lamports", amount);
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        msg!("I have closed the vault");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        space = 8 + 32,
        seeds = [b"state", maker.key().as_ref()],
        bump
    )]
    pub state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        seeds = [b"state", maker.key().as_ref()],
        bump
    )]
    pub state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        seeds = [b"state", maker.key().as_ref()],
        bump
    )]
    pub state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        close = maker,
        seeds = [b"state", maker.key().as_ref()],
        bump
    )]
    pub state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub maker: Pubkey,
}