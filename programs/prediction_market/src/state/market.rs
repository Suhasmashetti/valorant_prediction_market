use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarketStatus {
    Active,    // Market is open for betting
    Locked,    // Betting period has ended, awaiting resolution
    Resolved,  // Market has been resolved with a winner
    Cancelled, // Market was cancelled (e.g., match postponed)
}

impl Default for MarketStatus {
    fn default() -> Self {
        MarketStatus::Active
    }
}

#[account]
#[derive(Default)]
pub struct Market {
    pub id: u64,                     // Unique identifier
    pub name: String,                // Market name (e.g., "TSM vs Cloud9 - Valorant Champions 2025")
    pub description: String,         // Market description
    pub creator: Pubkey,             // Creator's public key
    pub outcomes: Vec<u8>,           // List of outcome IDs
    pub total_pool: u64,             // Total amount staked on this market
    pub resolved: bool,              // Whether the market has been resolved
    pub winner: Option<u8>,          // Winner outcome ID (if resolved)
    pub start_time: i64,             // Match start time (unix timestamp)
    pub end_time: i64,               // Market end time (betting closes)
    pub fee_percentage: u8,          // Platform fee percentage (e.g., 2 for 2%)
    pub oracle: Pubkey,              // Oracle authority that can resolve this market
    pub status: MarketStatus,        // Current market status
    pub bump: u8,                    // PDA bump
}