import express from "express";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hackthenest } from "../target/types/hackthenest";
import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { createMint, getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BN } from "bn.js";
import fs from "fs";

const app = express();
app.use(express.json());

// 1️⃣ Anchor setup
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.hackthenest as Program<Hackthenest>;
const connection = provider.connection;
const wallet = provider.wallet as anchor.Wallet;

// Optional: Load a keypair for server-side signer
const serverKeypair = Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(JSON.stringify([181,78,186,99,220,78,89,252,130,205,248,5,166,15,215,193,5,61,63,57,199,129,184,81,181,247,219,88,7,79,181,79,71,4,111,238,236,89,197,96,233,28,225,40,63,58,225,93,215,118,178,110,218,60,199,246,70,119,22,213,169,164,48,193])))
);

// ------------------------- API ENDPOINTS -------------------------

// Initialize program
app.post("/initialize", async (req, res) => {
  try {
    const tx = await program.methods.initialize().rpc();
    res.json({ success: true, tx });
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: err.toString() });
  }
});

// Mint tokens
app.post("/mint", async (req, res) => {
  try {
    const amount = new BN(req.body.amount);

    // Create mint
    const mint = await createMint(
      connection,
      serverKeypair,
      serverKeypair.publicKey,
      serverKeypair.publicKey,
      6
    );

    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      serverKeypair,
      mint,
      wallet.publicKey
    );

    await program.methods
      .mintTokens(amount)
      .accounts({
        signer: wallet.publicKey,
        mint,
        tokenAccount: ata.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    res.json({ success: true, mint: mint.toBase58(), ata: ata.address.toBase58(), amount: amount.toNumber() });
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: err.toString() });
  }
});

// Stake tokens
app.post("/stake", async (req, res) => {
  try {
    const amount = new BN(req.body.amount);
    const mintPubKey = new PublicKey(req.body.mint);

    const userAta = await getOrCreateAssociatedTokenAccount(
      connection,
      serverKeypair,
      mintPubKey,
      wallet.publicKey
    );

    await program.methods
      .stakeCoin(amount)
      .accounts({
        mint: mintPubKey,
        userTokenAccount: userAta.address,
        mintAuthority: wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([serverKeypair])
      .rpc();

    res.json({ success: true, stakedAmount: amount.toNumber() });
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: err.toString() });
  }
});

// Withdraw tokens
app.post("/withdraw", async (req, res) => {
  try {
    const mintPubKey = new PublicKey(req.body.mint);

    const sellerAta = await getOrCreateAssociatedTokenAccount(
      connection,
      serverKeypair,
      mintPubKey,
      wallet.publicKey
    );

    await program.methods
      .withdrawTokens()
      .accounts({
        seller: wallet.publicKey,
        mint: mintPubKey,
        sellerTokenAccount: sellerAta.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    res.json({ success: true });
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: err.toString() });
  }
});

// Start server
const PORT = 3000;
app.listen(PORT, () => console.log(`API running on http://localhost:${PORT}`));
