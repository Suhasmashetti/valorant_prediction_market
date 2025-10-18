use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserPosition {
    pub user: Pubkey,           // User's wallet address
    pub market: Pubkey,         // Market PDA address
    pub outcome: u8,            // ID of the outcome they bet on
    pub amount: u64,            // Amount of tokens staked
    pub shares: u64,            // Shares received (for AMM-based odds)
    pub timestamp: i64,         // When the position was created
    pub claimed: bool,          // Whether winnings have been claimed
    pub bump: u8,               // PDA bump
}