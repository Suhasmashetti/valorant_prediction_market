use anchor_lang::prelude::*;
use crate::state::{PlatformConfig, Market, MarketStatus};
use crate::errors::PredictionMarketError;

pub fn create_market(
    ctx: Context<CreateMarket>,
    name: String,
    description: String,
    start_time: i64,
    end_time: i64,
    custom_fee_percentage: Option<u8>,
) -> Result<()> {
    let platform_config = &mut ctx.accounts.platform_config;
    let market = &mut ctx.accounts.market;
    let creator = &ctx.accounts.creator;
    let clock = Clock::get()?;
    
    // Validation checks
    require!(
        !platform_config.paused,
        PredictionMarketError::PlatformPaused
    );
    
    require!(
        end_time > start_time,
        PredictionMarketError::InvalidTimeRange
    );
    
    require!(
        start_time > clock.unix_timestamp,
        PredictionMarketError::InvalidStartTime
    );
    
    // Initialize market data
    market.id = platform_config.markets_count;
    market.name = name;
    market.description = description;
    market.creator = creator.key();
    market.outcomes = Vec::new();
    market.total_pool = 0;
    market.resolved = false;
    market.winner = None;
    market.start_time = start_time;
    market.end_time = end_time;
    market.fee_percentage = custom_fee_percentage.unwrap_or(platform_config.default_fee_percentage);
    market.oracle = platform_config.oracle_authority;
    market.status = MarketStatus::Active;
    market.bump = ctx.bumps.market;
    
    platform_config.markets_count = platform_config.markets_count.checked_add(1).unwrap();
    
    msg!("Market created by admin: {}", market.name);
    msg!("Market ID: {}", market.id);
    msg!("Admin: {}", creator.key());

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    name: String,
    description: String,
    start_time: i64,
    end_time: i64,
    custom_fee_percentage: Option<u8>
)]
pub struct CreateMarket<'info> {
    #[account(
        mut,
        constraint = creator.key() == platform_config.admin @ PredictionMarketError::UnauthorizedAdmin
    )]
    pub creator: Signer<'info>,
    
    #[account(
        mut,
        seeds = [PlatformConfig::SEED_PREFIX.as_bytes()],
        bump = platform_config.bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    #[account(
        init,
        payer = creator,
        space = 8 +  // Discriminator
               8 +   // id
               4 + name.len() + // name (String)
               4 + description.len() + // description (String)
               32 +  // creator
               4 + 32 * 10 + // outcomes (Vec<u8> with estimated capacity)
               8 +   // total_pool
               1 +   // resolved
               1 + 1 + // winner (Option<u8>)
               8 +   // start_time
               8 +   // end_time
               1 +   // fee_percentage
               32 +  // oracle
               1 +   // status
               1,    // bump
        seeds = [b"market", platform_config.markets_count.to_le_bytes().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,
    
    pub system_program: Program<'info, System>,
}

