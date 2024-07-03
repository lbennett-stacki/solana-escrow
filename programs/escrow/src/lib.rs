use anchor_lang::prelude::*;

declare_id!("9XV6a4n8aavchcKE8NrFKJBNDHatTFizFF3bppAvn6sL");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
