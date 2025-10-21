use anchor_lang::prelude::*;
use crate::state::{Market, MarketStatus, PlatformConfig};
use crate::errors::PredictionMarketError;

pub fn resolve_market(ctx: Context<ResolveMarket>, winning_outcome_id: u8) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let oracle = &ctx.accounts.oracle;
    let clock = Clock::get()?;
    
    // Validations
    require!(
        market.oracle == oracle.key(),
        PredictionMarketError::UnauthorizedOracle
    );
    
    require!(
        market.status != MarketStatus::Resolved && market.status != MarketStatus::Cancelled,
        PredictionMarketError::MarketAlreadyResolved
    );
    
    require!(
        clock.unix_timestamp >= market.end_time,
        PredictionMarketError::MarketNotEnded
    );
    
    // Check that the winning outcome exists in this market
    let outcome_exists = market.outcomes.contains(&winning_outcome_id);
    require!(
        outcome_exists,
        PredictionMarketError::OutcomeNotFound
    );
    
    // Update market status
    market.status = MarketStatus::Resolved;
    market.resolved = true;
    market.winner = Some(winning_outcome_id);
    
    msg!("Market resolved: {}", market.name);
    msg!("Winning outcome ID: {}", winning_outcome_id);
    
    Ok(())
}

#[derive(Accounts)]
pub struct ResolveMarket<'info> {
    #[account(
        mut,
        constraint = market.status != MarketStatus::Resolved @ PredictionMarketError::MarketAlreadyResolved
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        constraint = market.oracle == oracle.key() @ PredictionMarketError::UnauthorizedOracle
    )]
    pub oracle: Signer<'info>,
    
    #[account(
        seeds = [PlatformConfig::SEED_PREFIX.as_bytes()],
        bump = platform_config.bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,
}