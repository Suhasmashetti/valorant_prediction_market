use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct PlatformConfig {
    pub admin: Pubkey,                // Platform administrator
    pub oracle_authority: Pubkey,     // Authority that can resolve markets
    pub treasury: Pubkey,             // Treasury account to collect fees
    pub default_fee_percentage: u8,   // Default platform fee (e.g., 2 for 2%)
    pub markets_count: u64,           // Total number of markets created
    pub total_volume: u64,            // Total volume across all markets
    pub paused: bool,                 // Whether the platform is paused
    pub bump: u8,                     // PDA bump
}

impl PlatformConfig {
    pub const SEED_PREFIX: &'static str = "platform-config";
    pub const SIZE: usize = 8 + // discriminator
                           32 + // admin
                           32 + // oracle_authority
                           32 + // treasury
                           1 +  // default_fee_percentage
                           8 +  // markets_count
                           8 +  // total_volume
                           1 +  // paused
                           1;   // bump
}