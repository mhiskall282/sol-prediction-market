import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import BN from "bn.js";

describe("jobmarketzk - end-to-end", () => {
  // Use local Anchor provider
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const program = anchor.workspace.myAnchorProject as Program<any>;

  it("create market, place bets, resolve and claim payout", async () => {
    // 1) Initialize config (admin = provider wallet)
    const configKeypair = Keypair.generate();
    const admin = provider.publicKey;
    const protocolFee = 100; // 1% default

    await program.methods
      .initialize(admin, protocolFee)
      .accounts({ config: configKeypair.publicKey, payer: admin, systemProgram: SystemProgram.programId })
      .signers([configKeypair])
      .rpc();

    console.log("Config initialized at", configKeypair.publicKey.toBase58());

    // 2) Create market
    const title = "Will ACME layoff >5% by 2025-12-31";
    const category = "LAYOFFS";
    const closeTime = Math.floor(Date.now() / 1000) + 60 * 60; // 1 hour from now
    const resolutionSource = "Company announcement";

    const [marketPda] = await PublicKey.findProgramAddress(
      [Buffer.from("market"), admin.toBuffer(), Buffer.from(title)],
      program.programId
    );

    const [treasuryPda] = await PublicKey.findProgramAddress(
      [Buffer.from("treasury"), marketPda.toBuffer()],
      program.programId
    );

    await program.methods
      .createMarket(title, category, new BN(closeTime), resolutionSource)
      .accounts({ admin: admin, config: configKeypair.publicKey, market: marketPda, treasuryVault: treasuryPda, systemProgram: SystemProgram.programId })
      .rpc();

    console.log("Market created:", marketPda.toBase58());

    // 3) Two users place bets
    const user1 = Keypair.generate();
    const user2 = Keypair.generate();

    // Airdrop to users on localnet/devnet
    await provider.connection.requestAirdrop(user1.publicKey, LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(user2.publicKey, LAMPORTS_PER_SOL);

    const amount1 = new BN(0.2 * LAMPORTS_PER_SOL); // user1 bets 0.2 SOL on YES
    const amount2 = new BN(0.1 * LAMPORTS_PER_SOL); // user2 bets 0.1 SOL on NO

    const [pos1Pda] = await PublicKey.findProgramAddress([Buffer.from("position"), marketPda.toBuffer(), user1.publicKey.toBuffer()], program.programId);
    const [pos2Pda] = await PublicKey.findProgramAddress([Buffer.from("position"), marketPda.toBuffer(), user2.publicKey.toBuffer()], program.programId);

    // user1 places YES bet (side = 0)
    await program.methods
      .placeBet(0, amount1)
      .accounts({ user: user1.publicKey, market: marketPda, userPosition: pos1Pda, treasuryVault: treasuryPda, systemProgram: SystemProgram.programId })
      .signers([user1])
      .rpc();

    // user2 places NO bet (side = 1)
    await program.methods
      .placeBet(1, amount2)
      .accounts({ user: user2.publicKey, market: marketPda, userPosition: pos2Pda, treasuryVault: treasuryPda, systemProgram: SystemProgram.programId })
      .signers([user2])
      .rpc();

    console.log("Bets placed");

    // 4) Close market
    await program.methods
      .closeMarket()
      .accounts({ admin: admin, market: marketPda })
      .rpc();

    // 5) Resolve market to YES (use 1 for YES)
    await program.methods
      .resolveMarket(1)
      .accounts({ admin: admin, market: marketPda, treasuryVault: treasuryPda })
      .rpc();

    console.log("Market resolved to YES");

    // 6) User1 claims payout
    const balanceBefore = await provider.connection.getBalance(user1.publicKey);

    await program.methods
      .claimPayout()
      .accounts({ user: user1.publicKey, market: marketPda, userPosition: pos1Pda, treasuryVault: treasuryPda, systemProgram: SystemProgram.programId })
      .signers([user1])
      .rpc();

    const balanceAfter = await provider.connection.getBalance(user1.publicKey);

    console.log("User1 balance before", balanceBefore, "after", balanceAfter);
    if (balanceAfter <= balanceBefore) {
      throw new Error("Payout was not received as expected");
    }

    // All good if we reach here
  });
});
