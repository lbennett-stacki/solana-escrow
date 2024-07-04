use anchor_lang::prelude::*;

declare_id!("7yAEaNyagncPhvNxJRmsURQ7AfihZPEFMTq4E3TnSLTq");

#[program]
pub mod simple_vault_escrow {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct Party {
    pub pubkey: Pubkey,       // 32
    pub withdraw_amount: u64, // 8
    pub deposit_amount: u64,  // 8
}

#[account]
pub struct SimpleVault {
    parties: [Party; 2], // 2 * (32 + 8 + 8) = 96
}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = 2 * (32 + 8 + 8))]
    pub vault: Account<'info, SimpleVault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, SimpleVault>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, SimpleVault>,
    #[account(mut)]
    pub withdrawer: Signer<'info>,
}
