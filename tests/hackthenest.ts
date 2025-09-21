import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hackthenest } from "../target/types/hackthenest";
import { BN } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { createMint, getOrCreateAssociatedTokenAccount, getAccount } from "@solana/spl-token";
import { expect } from "chai";

import {
  Connection,
  Keypair,
  SystemProgram,
  LAMPORTS_PER_SOL,
  Transaction,
  sendAndConfirmTransaction
} from "@solana/web3.js";



import idl from "../target/idl/hackthenest.json"
// import type { CounterProgram } from "@/types";
// import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import { Idl, AnchorProvider, setProvider } from "@coral-xyz/anchor";




before(() => {
  console.log("hello world")
}) 


describe("hackthenest", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  console.log("hello");

// const { connection } = useConnection();
// const wallet = useAnchorWallet();

  const connection = new Connection("https://api.devnet.solana.com", "confirmed");

//   const keypair = Keypair.generate();


//   const airdropSignature = await connection.requestAirdrop(
//     keypair.publicKey,
//     LAMPORTS_PER_SOL
//   )
//   await connection.confirmTransaction(airdropSignature);

//   const provider = new AnchorProvider(connection, keypair, {});


//   // const provider = new AnchorProvider(connection, wallet, {});
//   setProvider(provider);

// const program = new Program(idl as CounterProgram);

// // we can also explicitly mention the provider
// const program = new Program(idl as CounterProgram, provider);
// Anchor MethodsBuilder

  
//   const program = anchor.workspace.hackthenest as Program<Hackthenest>;
//   console.log("goodbye")
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });


it("creates a mint and mints to the payer", async () => {
    const provider = anchor.getProvider() as anchor.AnchorProvider;
    const wallet = provider.wallet as anchor.Wallet;

    const mint = await createMint(
      provider.connection,
      wallet.payer,
      wallet.publicKey,
      wallet.publicKey,
      6
    );

    const ata = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet.payer,
      mint,
      wallet.publicKey
    );

    const amount = new anchor.BN(1_000_000);
    await program.methods
      .mintTokens(amount)
      .accounts({
        signer: wallet.publicKey,
        mint,
        tokenAccount: ata.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const refreshed = await getAccount(provider.connection, ata.address);
    expect(Number(refreshed.amount)).to.equal(amount.toNumber());
  });



  
  
  // it("mint tokens", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.mint_tokens().rpc();
  //   console.log("Your transaction signature", tx);
  // });
});
