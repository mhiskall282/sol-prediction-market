// Migration deploy script: initialize config and create a sample market
import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import BN from "bn.js";

module.exports = async function (provider: anchor.AnchorProvider) {
  anchor.setProvider(provider);
  const program = anchor.workspace.myAnchorProject;
  const admin = provider.publicKey;

  // Create a config account
  const configKeypair = Keypair.generate();
  const protocolFee = 100; // 1%

  console.log("creating config account", configKeypair.publicKey.toBase58());
  await program.methods
    .initialize(admin, protocolFee)
    .accounts({ config: configKeypair.publicKey, payer: admin, systemProgram: anchor.web3.SystemProgram.programId })
    .signers([configKeypair])
    .rpc();

  // Optionally create a sample market (commented out by default)
  // const title = "Sample market";
  // const category = "MACRO LABOUR";
  // const closeTime = Math.floor(Date.now() / 1000) + 3600;
  // const resolutionSource = "News feed";

  // const [marketPda] = await PublicKey.findProgramAddress([
  //   Buffer.from("market"),
  //   admin.toBuffer(),
  //   Buffer.from(title),
  // ], program.programId);

  // const [treasuryPda] = await PublicKey.findProgramAddress([Buffer.from("treasury"), marketPda.toBuffer()], program.programId);

  // await program.methods
  //   .createMarket(title, category, new BN(closeTime), resolutionSource)
  //   .accounts({ admin: admin, config: configKeypair.publicKey, market: marketPda, treasuryVault: treasuryPda, systemProgram: anchor.web3.SystemProgram.programId })
  //   .rpc();

  console.log("migration complete");
};
