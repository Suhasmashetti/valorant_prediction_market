use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Token, Transfer};
use crate::state::{Market, Outcome, UserPosition, MarketStatus};
use crate::errors::PredictionMarketError;

pub fn place_bet(ctx: Context<PlaceBet>, amount: u64) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let outcome = &mut ctx.accounts.outcome;
    let user_position = &mut ctx.accounts.user_position;
    let clock = Clock::get()?;
    
    // Validations
    require!(
        market.status == MarketStatus::Active,
        PredictionMarketError::MarketNotActive
    );
    
    require!(
        clock.unix_timestamp < market.end_time,
        PredictionMarketError::MarketAlreadyClosed
    );
    
    require!(
        amount > 0,
        PredictionMarketError::InvalidBetAmount
    );
    
    // Transfer tokens from user to outcome's escrow account using transfer_checked
    let cpi_accounts = token::TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.escrow_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;
    
    // Calculate shares based on current odds
    // For simplicity, we'll use a 1:1 ratio initially
    let shares = amount;
    
    // Initialize or update user position
    user_position.user = ctx.accounts.user.key();
    user_position.market = market.key();
    user_position.outcome = outcome.id;
    user_position.amount = user_position.amount.checked_add(amount)
        .ok_or(PredictionMarketError::MathOverflow)?;
    user_position.shares = user_position.shares.checked_add(shares)
        .ok_or(PredictionMarketError::MathOverflow)?;
    user_position.timestamp = clock.unix_timestamp;
    user_position.claimed = false;
    user_position.bump = ctx.bumps.user_position;
    
    // Update outcome and market stats
    outcome.total_staked = outcome.total_staked.checked_add(amount)
        .ok_or(PredictionMarketError::MathOverflow)?;
    market.total_pool = market.total_pool.checked_add(amount)
        .ok_or(PredictionMarketError::MathOverflow)?;
    
    // Update odds (simplified version - can be more complex in reality)
    // In a real implementation, this would adjust based on relative liquidity
    
    msg!("Bet placed on outcome: {}", outcome.name);
    msg!("Amount: {}", amount);
    
    Ok(())
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        constraint = market.status == MarketStatus::Active @ PredictionMarketError::MarketNotActive
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        mut,
        seeds = [b"outcome", market.key().as_ref(), outcome.id.to_le_bytes().as_ref()],
        bump = outcome.bump
    )]
    pub outcome: Account<'info, Outcome>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 +   // Discriminator
               32 +   // user
               32 +   // market
               1 +    // outcome
               8 +    // amount
               8 +    // shares
               8 +    // timestamp
               1 +    // claimed
               1,     // bump
        seeds = [
            b"user_position",
            user.key().as_ref(),
            market.key().as_ref(),
            outcome.id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub user_position: Account<'info, UserPosition>,
    
    /// The mint of the token being bet
    pub mint: Account<'info, token::Mint>,
    
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
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
