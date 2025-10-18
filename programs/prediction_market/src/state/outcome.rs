use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Outcome {
    pub id: u8,                 // Unique identifier within a market
    pub market: Pubkey,         // The market this outcome belongs to
    pub name: String,           // Name of the outcome (e.g., "TSM wins")
    pub escrow_pubkey: Pubkey,  // PDA that holds staked tokens for this outcome
    pub total_staked: u64,      // Total tokens staked on this outcome
    pub odds: u64,              // Current odds (represented as integer, actual odds = odds/10000)
    pub bump: u8,               // PDA bump
}