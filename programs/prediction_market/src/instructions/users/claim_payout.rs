use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Token, Mint};
use crate::state::{Market, Outcome, UserPosition, MarketStatus};
use crate::errors::PredictionMarketError;

pub fn claim_payout(ctx: Context<ClaimPayout>) -> Result<()> {
    let market = &ctx.accounts.market;
    let user_position = &mut ctx.accounts.user_position;
    let _winner_outcome = &ctx.accounts.outcome;
    
    // Validations
    require!(
        market.status == MarketStatus::Resolved,
        PredictionMarketError::MarketNotResolved
    );
    
    require!(
        !user_position.claimed,
        PredictionMarketError::AlreadyClaimed
    );
    
    require!(
        market.winner.unwrap() == user_position.outcome,
        PredictionMarketError::NotWinner
    );
    
    // Calculate payout based on pool and user's share
    let total_market_pool = market.total_pool;
    let fee_amount = (total_market_pool * market.fee_percentage as u64) / 100;
    let distributable_pool = total_market_pool.checked_sub(fee_amount)
        .ok_or(PredictionMarketError::MathOverflow)?;

    // Calculate user's share based on their proportion of winning bets
    let total_winning_outcome_staked = ctx.accounts.outcome.total_staked;
    
    // Prevent division by zero
    require!(
        total_winning_outcome_staked > 0,
        PredictionMarketError::InsufficientLiquidity
    );
    
    // Calculate payout proportional to user's contribution to winning pool
    let user_share_numerator = user_position.amount;
    let user_share_denominator = total_winning_outcome_staked;
    
    // Use checked math to avoid overflows
    let payout = (user_share_numerator as u128)
        .checked_mul(distributable_pool as u128)
        .ok_or(PredictionMarketError::MathOverflow)?
        .checked_div(user_share_denominator as u128)
        .ok_or(PredictionMarketError::MathOverflow)? as u64;
    
    // Transfer tokens from escrow to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.escrow_authority.to_account_info(),
    };
    
    // Create the CPI context with signer seeds for the escrow PDA
    let bump = [ctx.bumps.escrow_authority];
    let market_key = market.key();
    let outcome_id_bytes = ctx.accounts.outcome.id.to_le_bytes();
    
    let escrow_seeds = &[
        b"escrow",
        market_key.as_ref(),
        outcome_id_bytes.as_ref(),
        &bump
    ];
    
    let signer_seeds = &[&escrow_seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds
    );
    
    // Execute the transfer
    token::transfer(cpi_ctx, payout)?;
    
    // Mark position as claimed
    user_position.claimed = true;
    
    msg!("Payout claimed: {}", payout);
    
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimPayout<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        constraint = market.status == MarketStatus::Resolved @ PredictionMarketError::MarketNotResolved
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        constraint = outcome.id == market.winner.unwrap() @ PredictionMarketError::OutcomeNotFound,
        seeds = [b"outcome", market.key().as_ref(), outcome.id.to_le_bytes().as_ref()],
        bump = outcome.bump
    )]
    pub outcome: Account<'info, Outcome>,
    
    #[account(
        mut,
        seeds = [
            b"user_position",
            user.key().as_ref(),
            market.key().as_ref(),
            outcome.id.to_le_bytes().as_ref()
        ],
        bump = user_position.bump,
        constraint = !user_position.claimed @ PredictionMarketError::AlreadyClaimed,
        constraint = user_position.user == user.key() @ PredictionMarketError::UnauthorizedAdmin
    )]
    pub user_position: Account<'info, UserPosition>,
    
    /// The mint of the token being used for payouts
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ PredictionMarketError::UnauthorizedAdmin,
        constraint = user_token_account.mint == mint.key() @ PredictionMarketError::InvalidMint
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"escrow", market.key().as_ref(), outcome.id.to_le_bytes().as_ref()],
        bump,
        constraint = escrow_token_account.mint == mint.key() @ PredictionMarketError::InvalidMint
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: This is the PDA that has authority over the escrow
    #[account(
        seeds = [b"escrow", market.key().as_ref(), outcome.id.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow_authority: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}