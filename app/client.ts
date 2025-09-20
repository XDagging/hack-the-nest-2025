import * as anchor from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { Keypair, PublicKey } from "@solana/web3.js";

// Load Anchor provider


async function testFunc() {
    const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

// Load the program IDL
const program = anchor.workspace.MyTokenProject;

// Create a new mint keypair
const mintKeypair = Keypair.generate();

// Create associated token account for your wallet
const destinationAccount = await getOrCreateAssociatedTokenAccount(
  provider.connection,
  provider.wallet.payer,
  mintKeypair.publicKey,
  provider.wallet.publicKey
);

// Call the create_mint function
await program.methods.createMint(new anchor.BN(9)).accounts({
  mint: mintKeypair.publicKey,
  user: provider.wallet.publicKey,
  mintAuthority: provider.wallet.publicKey, // wallet is authority
  systemProgram: anchor.web3.SystemProgram.programId,
  tokenProgram: TOKEN_PROGRAM_ID,
  rent: anchor.web3.SYSVAR_RENT_PUBKEY,
}).signers([mintKeypair]).rpc();

console.log("Mint created:", mintKeypair.publicKey.toBase58());

// Mint 1000 tokens to your wallet
await program.methods.mintTokens(new anchor.BN(1000)).accounts({
  mint: mintKeypair.publicKey,
  destination: destinationAccount.address,
  mintAuthority: provider.wallet.publicKey,
  tokenProgram: TOKEN_PROGRAM_ID,
}).rpc();

console.log("Minted 1000 tokens!");



}
