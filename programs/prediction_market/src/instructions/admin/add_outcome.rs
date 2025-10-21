use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::{Market, Outcome, MarketStatus};
use crate::errors::PredictionMarketError;

pub fn add_outcome(ctx: Context<AddOutcome>, name: String, outcome_id: u8) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let outcome = &mut ctx.accounts.outcome;
    
    require!(
        market.status == MarketStatus::Active,
        PredictionMarketError::MarketNotActive
    );
    
    // Initialize outcome
    outcome.id = outcome_id;
    outcome.market = market.key();
    outcome.name = name;
    outcome.escrow_pubkey = ctx.accounts.escrow_token_account.key();
    outcome.total_staked = 0;
    outcome.odds = 10000; // Default 1:1 odds (represented as 1.0000)
    outcome.bump = ctx.bumps.outcome;
    
    // Add outcome ID to market's outcomes list
    market.outcomes.push(outcome.id);
    
    msg!("Outcome added: {}", outcome.name);
    msg!("Outcome ID: {}", outcome.id);
    msg!("Escrow token account created: {}", ctx.accounts.escrow_token_account.key());
    
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
    
    /// CHECK: This is the PDA that will have authority over the escrow
    #[account(
        seeds = [b"escrow", market.key().as_ref(), &outcome_id.to_le_bytes()],
        bump,
    )]
    pub escrow_authority: UncheckedAccount<'info>,
    
    /// The token mint for the escrow account
    pub mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = escrow_authority,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}