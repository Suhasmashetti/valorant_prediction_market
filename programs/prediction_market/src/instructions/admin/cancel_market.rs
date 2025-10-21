use anchor_lang::prelude::*;
use crate::state::{Market, MarketStatus, PlatformConfig};
use crate::errors::PredictionMarketError;

pub fn cancel_market(ctx: Context<CancelMarket>) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let admin = &ctx.accounts.admin;
    
    // Validations
    require!(
        market.status != MarketStatus::Resolved && market.status != MarketStatus::Cancelled,
        PredictionMarketError::MarketAlreadyResolved
    );
    
    // Update market status
    market.status = MarketStatus::Cancelled;
    
    msg!("Market cancelled: {}", market.name);
    msg!("Cancelled by admin: {}", admin.key());
    
    Ok(())
}

#[derive(Accounts)]
pub struct CancelMarket<'info> {
    #[account(
        mut,
        constraint = market.status != MarketStatus::Cancelled @ PredictionMarketError::MarketAlreadyResolved
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        seeds = [PlatformConfig::SEED_PREFIX.as_bytes()],
        bump = platform_config.bump,
        constraint = platform_config.admin == admin.key() @ PredictionMarketError::UnauthorizedAdmin
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
}