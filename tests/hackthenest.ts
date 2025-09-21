import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hackthenest } from "../target/types/hackthenest";
import { BN } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  getAccount,
} from "@solana/spl-token";
import { expect } from "chai";

import {
  Connection,
  SystemProgram,
  PublicKey,
} from "@solana/web3.js";

anchor.setProvider(anchor.AnchorProvider.env());
const provider = anchor.getProvider() as anchor.AnchorProvider;
const wallet = provider.wallet as anchor.Wallet;
const connection = provider.connection;
const program = anchor.workspace.hackthenest as Program<Hackthenest>;

before(() => {
  console.log("Starting hackthenest tests...");
});

describe("hackthenest", () => {
  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Initialization transaction signature:", tx);
  });

  it("creates a mint and mints to the payer", async () => {
    const mint = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      wallet.publicKey,
      6
    );

    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      mint,
      wallet.publicKey
    );

    const amount = new BN(1_000_000);
    await program.methods
      .mintTokens(amount)
      .accounts({
        signer: wallet.publicKey,
        mint,
        tokenAccount: ata.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const refreshed = await getAccount(connection, ata.address);
    expect(Number(refreshed.amount)).to.equal(amount.toNumber());
  });

  it("Stakes an event", async () => {
    const mint = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      wallet.publicKey,
      6
    );

    const userAta = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      mint,
      wallet.publicKey
    );

    const amount = new BN(1_000_000);

    await program.methods
      .stakeCoin(amount)
      .accounts({
        mint,
        userTokenAccount: userAta.address,
        mintAuthority: wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([wallet.payer])
      .rpc();

    const refreshedAta = await getAccount(connection, userAta.address);
    console.log("Staked amount in ATA:", Number(refreshedAta.amount));
    expect(Number(refreshedAta.amount)).to.equal(amount.toNumber());
  });

  it("Withdrawing coins", async () => {
    // 1️⃣ Create mint and seller ATA
    const mint = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      wallet.publicKey,
      6
    );

    const sellerTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      mint,
      wallet.publicKey
    );

    // 2️⃣ Mint tokens to seller
    const mintAmount = new BN(1_000_000);
await program.methods
  .mintTokens(mintAmount)
  .accounts({
    signer: wallet.publicKey,
    mint,
    tokenAccount: sellerTokenAccount.address,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .rpc();
   
    // 3️⃣ Generate treasury and registry PDAs
    const [treasury] = await PublicKey.findProgramAddress(
      [Buffer.from("treasury")],
      program.programId
    );

    const [registry] = await PublicKey.findProgramAddress(
      [Buffer.from("registry_seed")],
      program.programId
    );

    const [mintAuthority] = await PublicKey.findProgramAddress(
  [Buffer.from("mint_auth")], // match seeds in Rust
  program.programId
);

    // 4️⃣ Check initial balances
    const tokenBefore = await connection.getTokenAccountBalance(sellerTokenAccount.address);
    console.log("Seller token balance before:", tokenBefore.value.amount);

    const solBefore = await connection.getBalance(wallet.publicKey);
    console.log("Seller SOL balance before:", solBefore);

    // 5️⃣ Call withdrawTokens
    await program.methods
  .withdrawTokens()
  .accounts({
    seller: wallet.publicKey,               // signer
    mint,
    sellerTokenAccount: sellerTokenAccount.address,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .rpc();




    // 6️⃣ Check token balance after withdraw
    const tokenAfter = await connection.getTokenAccountBalance(sellerTokenAccount.address);
    console.log("Seller token balance after:", tokenAfter.value.amount);
    expect(tokenAfter.value.amount).to.equal("0");

    // 7️⃣ Check SOL balance after withdraw
    const solAfter = await connection.getBalance(wallet.publicKey);
    console.log("Seller SOL balance after:", solAfter);
    expect(solAfter).to.be.greaterThan(solBefore);
  });
});
