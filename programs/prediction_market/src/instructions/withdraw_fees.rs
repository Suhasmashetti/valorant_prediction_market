use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Token, TransferChecked};
use crate::state::PlatformConfig;
use crate::errors::PredictionMarketError;

pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
    let platform_config = &ctx.accounts.platform_config;
    
    // Get the current balance of the treasury account
    let amount = ctx.accounts.treasury_token_account.amount;
    
    require!(
        amount > 0,
        PredictionMarketError::InsufficientLiquidity
    );
    
    // Transfer tokens from treasury to admin using transfer_checked
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        to: ctx.accounts.admin_token_account.to_account_info(),
        authority: ctx.accounts.treasury_authority.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };
    
    // Create the CPI context with signer seeds for the treasury PDA
    let bump = [ctx.bumps.treasury_authority];
    let treasury_seeds = &[
        PlatformConfig::SEED_PREFIX.as_bytes(),
        b"treasury",
        &bump
    ];
    
    let signer_seeds = &[&treasury_seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds
    );
    
    // Execute the transfer
    token::transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;
    
    msg!("Fees withdrawn: {}", amount);
    
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(
        constraint = platform_config.admin == admin.key() @ PredictionMarketError::UnauthorizedAdmin,
        seeds = [PlatformConfig::SEED_PREFIX.as_bytes()],
        bump = platform_config.bump
    )]
    pub platform_config: Account<'info, PlatformConfig>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// The mint of the token being used for fees
    pub mint: Account<'info, token::Mint>,
    
    #[account(
        mut,
        constraint = admin_token_account.owner == admin.key() @ PredictionMarketError::UnauthorizedAdmin,
        constraint = admin_token_account.mint == mint.key() @ PredictionMarketError::InvalidMint
    )]
    pub admin_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = treasury_token_account.mint == mint.key() @ PredictionMarketError::InvalidMint,
        constraint = platform_config.treasury == treasury_token_account.key() @ PredictionMarketError::UnauthorizedAdmin
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: This is the PDA that has authority over the treasury
    #[account(
        seeds = [PlatformConfig::SEED_PREFIX.as_bytes(), b"treasury"],
        bump
    )]
    pub treasury_authority: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
