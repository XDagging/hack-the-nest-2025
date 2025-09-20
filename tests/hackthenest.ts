import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hackthenest } from "../target/types/hackthenest";
import { BN } from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

before(() => {
  console.log("hello world")
}) 


describe("hackthenest", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  console.log("hello")

  
  const program = anchor.workspace.hackthenest as Program<Hackthenest>;
  console.log("goodbye")
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });


  it("creating mint", async () => {
    // Add your test here.
    const tx = await program.methods.createMint().rpc();
    
    const connection = anchor.getProvider().connection;

    

    const airdropAmt = 100000*anchor.web3.LAMPORTS_PER_SOL;

    const x = anchor.getProvider().publicKey;

    console.log("this is the x", x);

    const airdropTxHash = await connection.requestAirdrop(x, airdropAmt);
  
    const amount = new BN(100); 
    const y = await program.methods.buyTokens(amount).accounts({
      tokenProgram: TOKEN_PROGRAM_ID

    }).rpc();
    console.log("Your transaction signature", y);
  });

  
  // it("mint tokens", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.mint_tokens().rpc();
  //   console.log("Your transaction signature", tx);
  // });
});
