use anchor_lang::prelude::*;

declare_id!("6cU6Lvy6xzZGDTVP52P21ku6iguJt3oJkY2jdsCCM3TJ");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
