use anchor_lang::prelude::*;

declare_id!("4asst9oqh9cAryCAViQ2pySSESqP9TLd5nEaz5BJfrxL");

#[program]
pub mod prediction_market {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
