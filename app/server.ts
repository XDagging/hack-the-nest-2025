// server.ts
import express from "express";
import { program, PublicKey, provider } from "./client";

const app = express();
app.use(express.json());

// Example endpoint: withdraw tokens
app.post("/withdraw", async (req, res) => {
  try {
    const { sellerPubkey, sellerTokenAccount, mint } = req.body;

    const tx = await program.methods
      .withdrawTokens()
      .accounts({
        seller: new PublicKey(sellerPubkey),
        mint: new PublicKey(mint),
        sellerTokenAccount: new PublicKey(sellerTokenAccount),
        tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
      })
      .rpc();

    res.json({ success: true, tx });
  } catch (err: any) {
    console.error(err);
    res.status(500).json({ success: false, error: err.toString() });
  }
});

// Example endpoint: stake tokens
app.post("/stake", async (req, res) => {
  try {
    const { userTokenAccount, mint, amount } = req.body;

    const tx = await program.methods
      .stakeCoin(new anchor.BN(amount))
      .accounts({
        mint: new PublicKey(mint),
        userTokenAccount: new PublicKey(userTokenAccount),
        mintAuthority: provider.wallet.publicKey,
        tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
      })
      .rpc();

    res.json({ success: true, tx });
  } catch (err: any) {
    console.error(err);
    res.status(500).json({ success: false, error: err.toString() });
  }
});

app.listen(3000, () => {
  console.log("Backend running on http://localhost:3000");
});
