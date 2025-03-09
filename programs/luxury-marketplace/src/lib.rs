use anchor_lang::prelude::*;

declare_id!("bVwtqr5iYzMPhiJWLsRFRre2QjjSjR6v7ksN2Z2ZzHY");

#[program]
pub mod luxury_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
