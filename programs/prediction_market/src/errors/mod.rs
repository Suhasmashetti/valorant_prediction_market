use anchor_lang::prelude::*;

#[error_code]
pub enum PredictionMarketError {
    #[msg("Platform is currently paused")]
    PlatformPaused,
    
    #[msg("End time must be after start time")]
    InvalidTimeRange,
    
    #[msg("Market start time must be in the future")]
    InvalidStartTime,
    
    #[msg("Market is not in active state")]
    MarketNotActive,
    
    #[msg("Market is already closed")]
    MarketAlreadyClosed,
    
    #[msg("Market is not resolved yet")]
    MarketNotResolved,
    
    #[msg("Invalid bet amount")]
    InvalidBetAmount,
    
    #[msg("Unauthorized oracle")]
    UnauthorizedOracle,
    
    #[msg("Position already claimed")]
    AlreadyClaimed,
    
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    
    #[msg("Outcome not found")]
    OutcomeNotFound,
    
    #[msg("Not the bet winner")]
    NotWinner,
    
    #[msg("Unauthorized admin")]
    UnauthorizedAdmin,
    
    #[msg("Market has not reached end time")]
    MarketNotEnded,
    
    #[msg("Math overflow")]
    MathOverflow,
    
    #[msg("Invalid mint, expected a different token mint")]
    InvalidMint,
    
    #[msg("Market is already resolved")]
    MarketAlreadyResolved,
}