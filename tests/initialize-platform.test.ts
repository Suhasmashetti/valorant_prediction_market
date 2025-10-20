import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("Initialize Platform", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Access the program using the workspace
  const program = anchor.workspace.PredictionMarket;
  
  // Admin keypair for testing
  const admin = anchor.web3.Keypair.generate();
  // Treasury account for collecting fees
  const treasury = anchor.web3.Keypair.generate();
  
  // Generate the platform config PDA
  const [platformConfigPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("platform-config")],
    program.programId
  );

  before(async () => {
    // Airdrop SOL to admin for gas fees
    const signature = await provider.connection.requestAirdrop(
      admin.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(signature);
  });

  it("Should initialize platform configuration", async () => {
    try {
      // Prepare the accounts for the instruction
      const accounts = {
        admin: admin.publicKey,
        treasury: treasury.publicKey,
        platformConfig: platformConfigPDA, // Using camelCase as required by Anchor
        systemProgram: SystemProgram.programId,
      };

      // Execute the initialize_platform instruction
      const tx = await program.methods
        .initializePlatform()
        .accounts(accounts)
        .signers([admin])
        .rpc();
      
      console.log("Platform initialization transaction:", tx);
      
      // Fetch the platform configuration to verify it was created correctly
      const platformConfig = await program.account.platformConfig.fetch(platformConfigPDA);
      
      // Print out the platformConfig to see what we got back
      console.log("Platform Config:", platformConfig);
      
      // Verify the platform configuration was set correctly using the actual field names
      assert.ok(platformConfig.admin.toBase58() === admin.publicKey.toBase58(), "Admin key should match");
      assert.ok(platformConfig.oracleAuthority.toBase58() === admin.publicKey.toBase58(), "Oracle authority should initially be the admin");
      assert.ok(platformConfig.treasury.toBase58() === treasury.publicKey.toBase58(), "Treasury key should match");
      assert.equal(platformConfig.defaultFeePercentage, 2, "Default fee percentage should be 2%");
      assert.equal(platformConfig.marketsCount.toNumber(), 0, "Markets count should start at 0");
      assert.equal(platformConfig.totalVolume.toNumber(), 0, "Total volume should start at 0");
      assert.equal(platformConfig.paused, false, "Platform should not start paused");
      
      console.log("Platform configuration initialized and verified successfully");
    } catch (error) {
      console.error("Test failed with error:", error);
      throw error;
    }
  });
});