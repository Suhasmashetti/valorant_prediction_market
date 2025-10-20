# Valorant Prediction Market

A decentralized prediction market platform for esports matches built on Solana. This platform allows users to bet on the outcomes of Valorant tournaments and matches, with transparent odds and automated payouts.

## Overview

This platform is similar to Polymarket but focused specifically on esports betting. Users can place bets on their favorite teams in upcoming matches, and winners receive payouts automatically through smart contracts once the match results are determined by a trusted oracle.

## Features

- **Decentralized Betting**: Place bets directly through Solana blockchain transactions
- **Multiple Markets**: Create and participate in various esports match markets
- **Transparent Odds**: All odds are calculated based on the pool of bets
- **Automatic Payouts**: Winners can claim their rewards automatically after match resolution
- **Fee Structure**: Configurable platform fees (default: 2%)

## Smart Contract Architecture

The platform consists of the following key components:

### Core Accounts

1. **PlatformConfig**: Global settings and admin controls
2. **Market**: Individual prediction markets for specific matches
3. **Outcome**: Possible outcomes within a market (teams)
4. **UserPosition**: User's bet on a specific outcome

### Main Instructions

1. **initialize_platform**: Set up the platform configuration
2. **create_market**: Create a new market for an upcoming match
3. **add_outcome**: Add teams/outcomes to a market
4. **place_bet**: Place a bet on a specific team
5. **resolve_market**: Resolve the market with the winning team (oracle only)
6. **claim_payout**: Claim winnings (winners only)
7. **withdraw_fees**: Withdraw platform fees (admin only)

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solanalabs.com/cli/install)
- [Anchor Framework](https://project-serum.github.io/anchor/getting-started/installation.html)

### Installation

1. Clone the repository
   ```bash
   git clone https://github.com/Suhasmashetti/valorant_prediction_market.git
   cd valorant_prediction_market
   ```

2. Install dependencies
   ```bash
   npm install
   ```

3. Build the program
   ```bash
   anchor build
   ```

4. Test the program
   ```bash
   anchor test
   ```

### Deployment

1. Deploy to devnet
   ```bash
   anchor deploy --provider.cluster devnet
   ```

2. Update program ID
   - Replace the program ID in `Anchor.toml` and `lib.rs` with your deployed program ID

## How it Works

### Creating a Market

Only the platform admin can create markets:

```typescript
// Example of creating a new market
const txSignature = await program.methods.createMarket(
  "TSM vs Cloud9 - VCT Finals 2025",
  "Valorant Champions Tour Grand Finals match between TSM and Cloud9",
  startTimestamp,
  endTimestamp,
  null // use default fee percentage
).accounts({
  creator: adminWallet.publicKey,
  platformConfig: platformConfigAddress,
  market: marketAddress,
  systemProgram: anchor.web3.SystemProgram.programId,
}).rpc();
```

### Adding Outcomes

After creating a market, add the possible outcomes (teams):

```typescript
// Add first team
await program.methods.addOutcome(
  "TSM"
).accounts({
  market: marketAddress,
  outcomeId: 0,
  authority: adminWallet.publicKey,
  outcome: outcomeAddress,
  escrow: escrowAddress,
  systemProgram: anchor.web3.SystemProgram.programId,
}).rpc();

// Add second team
await program.methods.addOutcome(
  "Cloud9"
).accounts({
  market: marketAddress,
  outcomeId: 1,
  authority: adminWallet.publicKey,
  outcome: outcome2Address,
  escrow: escrow2Address,
  systemProgram: anchor.web3.SystemProgram.programId,
}).rpc();
```

### Placing Bets

Users can place bets on their chosen team:

```typescript
await program.methods.placeBet(
  new anchor.BN(1000000) // 1 SOL (or equivalent token amount)
).accounts({
  user: userWallet.publicKey,
  market: marketAddress,
  outcome: outcomeAddress,
  userPosition: userPositionAddress,
  mint: mintAddress,
  userTokenAccount: userTokenAccount,
  escrowTokenAccount: escrowTokenAccount,
  tokenProgram: TOKEN_PROGRAM_ID,
  systemProgram: anchor.web3.SystemProgram.programId,
}).rpc();
```

### Resolving Markets

Only the authorized oracle can resolve a market:

```typescript
await program.methods.resolveMarket(
  0 // Winner ID (0 for TSM in this example)
).accounts({
  market: marketAddress,
  oracle: oracleWallet.publicKey,
  platformConfig: platformConfigAddress,
}).rpc();
```

### Claiming Payouts

Winners can claim their payouts after market resolution:

```typescript
await program.methods.claimPayout().accounts({
  user: userWallet.publicKey,
  market: marketAddress,
  outcome: outcomeAddress,
  userPosition: userPositionAddress,
  mint: mintAddress,
  userTokenAccount: userTokenAccount,
  escrowTokenAccount: escrowTokenAccount,
  escrowAuthority: escrowAuthorityAddress,
  tokenProgram: TOKEN_PROGRAM_ID,
  systemProgram: anchor.web3.SystemProgram.programId,
}).rpc();
```

## Payout Calculation

The payout is calculated proportionally based on the user's contribution to the winning outcome pool:

```
total_market_pool = sum of all bets on all outcomes
fee_amount = total_market_pool * fee_percentage / 100
distributable_pool = total_market_pool - fee_amount

user_share = user_bet_amount / total_winning_outcome_staked
payout = user_share * distributable_pool
```

### Example:
- Total market pool: 1,000,000 tokens
- Fee percentage: 2%
- Platform fees: 20,000 tokens
- Distributable pool: 980,000 tokens
- Total staked on winning outcome: 400,000 tokens
- User's bet on winning outcome: 100,000 tokens
- User's payout: (100,000/400,000) * 980,000 = 245,000 tokens
- User's profit: 245,000 - 100,000 = 145,000 tokens

## Security Considerations

- Oracle authority is trusted for accurate result reporting
- Admin controls are limited to platform management and fee withdrawal
- Users can only claim payouts for winning bets they placed
- All token transfers use `transfer_checked` for enhanced security

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

- Inspired by Polymarket and other prediction market platforms
- Built using the Anchor Framework and Solana blockchain