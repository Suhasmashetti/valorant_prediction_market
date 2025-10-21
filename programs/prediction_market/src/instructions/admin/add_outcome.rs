use anchor_lang::prelude::*;
use crate::state::{Market, Outcome, MarketStatus};
use crate::errors::PredictionMarketError;

pub fn add_outcome(ctx: Context<AddOutcome>, name: String, outcome_id: u8) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let outcome = &mut ctx.accounts.outcome;
    
    // Ensure market is still active and not resolved
    require!(
        market.status == MarketStatus::Active,
        PredictionMarketError::MarketNotActive
    );
    
    // Initialize outcome
    outcome.id = outcome_id;
    outcome.market = market.key();
    outcome.name = name;
    outcome.escrow_pubkey = ctx.accounts.escrow.key();
    outcome.total_staked = 0;
    outcome.odds = 10000; // Default 1:1 odds (represented as 1.0000)
    outcome.bump = ctx.bumps.outcome;
    
    // Add outcome ID to market's outcomes list
    market.outcomes.push(outcome.id);
    
    msg!("Outcome added: {}", outcome.name);
    msg!("Outcome ID: {}", outcome.id);
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String, outcome_id: u8)]
pub struct AddOutcome<'info> {
    #[account(
        mut,
        constraint = market.status == MarketStatus::Active @ PredictionMarketError::MarketNotActive
    )]
    pub market: Account<'info, Market>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 +   // Discriminator
               1 +    // id
               32 +   // market
               4 + name.len() + // name (String)
               32 +   // escrow_pubkey
               8 +    // total_staked
               8 +    // odds (u64)
               1,     // bump
        seeds = [b"outcome", market.key().as_ref(), &outcome_id.to_le_bytes()],
        bump
    )]
    pub outcome: Account<'info, Outcome>,
    
    /// CHECK: This is the escrow account that will hold staked tokens
    #[account(
        seeds = [b"escrow", market.key().as_ref(), &outcome_id.to_le_bytes()],
        bump,
    )]
    pub escrow: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}