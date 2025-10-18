use anchor_lang::prelude::*;
use crate::state::PlatformConfig;

pub fn initialize_platform(ctx: Context<InitializePlatform>) -> Result<()> {
    let platform_config = &mut ctx.accounts.platform_config;
    let admin = &ctx.accounts.admin;

    // Initialize platform configuration
    platform_config.admin = admin.key();
    platform_config.oracle_authority = admin.key(); // Initially set oracle to admin, can be changed later
    platform_config.treasury = ctx.accounts.treasury.key();
    platform_config.default_fee_percentage = 2; // Default 2% fee
    platform_config.markets_count = 0;
    platform_config.total_volume = 0;
    platform_config.paused = false;
    platform_config.bump = ctx.bumps.platform_config;

    msg!("Platform initialized with admin: {:?}", admin.key());
    msg!("Platform treasury set to: {:?}", ctx.accounts.treasury.key());

    Ok(())
}

#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// CHECK: This is the treasury account that will receive fees
    pub treasury: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = admin,
        space = PlatformConfig::SIZE,
        seeds = [PlatformConfig::SEED_PREFIX.as_bytes()],
        bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    pub system_program: Program<'info, System>,
}