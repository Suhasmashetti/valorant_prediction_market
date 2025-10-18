use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;
use state::*;
use errors::PredictionMarketError;

declare_id!("4asst9oqh9cAryCAViQ2pySSESqP9TLd5nEaz5BJfrxL");

#[program]
pub mod prediction_market {
    use super::*;

    // Platform management
    pub fn initialize_platform(ctx: Context<InitializePlatform>) -> Result<()> {
        instructions::initialize_platform(ctx)
    }

    // Market management
    pub fn create_market(
        ctx: Context<CreateMarket>,
        name: String,
        description: String,
        start_time: i64,
        end_time: i64,
        custom_fee_percentage: Option<u8>,
    ) -> Result<()> {
        instructions::create_market(ctx, name, description, start_time, end_time, custom_fee_percentage)
    }

    pub fn add_outcome(ctx: Context<AddOutcome>, name: String, outcome_id: u8) -> Result<()> {
        instructions::add_outcome(ctx, name, outcome_id)
    }

    // Betting functions
    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64) -> Result<()> {
        instructions::place_bet(ctx, amount)
    }

    pub fn resolve_market(ctx: Context<ResolveMarket>, winning_outcome_id: u8) -> Result<()> {
        instructions::resolve_market(ctx, winning_outcome_id)
    }

    pub fn claim_payout(ctx: Context<ClaimPayout>) -> Result<()> {
        instructions::claim_payout(ctx)
    }

    // Admin functions
    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
        instructions::withdraw_fees(ctx)
    }
}
