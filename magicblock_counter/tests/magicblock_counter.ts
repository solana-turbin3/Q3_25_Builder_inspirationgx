import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MagicblockCounter } from "../target/types/magicblock_counter";
import user_file from "./wallets/user-1-wallet.json";

const user_wallet = anchor.web3.Keypair.fromSecretKey(
  new Uint8Array(user_file)
);

describe("magicblock_counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .magicblockCounter as Program<MagicblockCounter>;

  it.skip("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        user: user_wallet.publicKey
      })
      .signers([user_wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("should increase counter normally", async () => {
    // Add your test here.
    const tx = await program.methods
      .increment()
      .accounts({
        user: user_wallet.publicKey
      })
      .signers([user_wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("should delegate counter to magicblock", async () => {
    // Add your test here.
    const tx = await program.methods
      .delegate()
      .accounts({
        user: user_wallet.publicKey
      })
      .signers([user_wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("should fail to increase counter normally", async () => {
    // Add your test here.
    const tx = await program.methods
      .increment()
      .accounts({
        user: user_wallet.publicKey
      })
      .signers([user_wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("should commit counter account state to delegated magicblock", async () => {
    // Add your test here.
    const tx = await program.methods
      .commit()
      .accounts({
        user: user_wallet.publicKey
      })
      .signers([user_wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("should increase counter and commit counter account state to delegated magicblock", async () => {
    // Add your test here.
    const tx = await program.methods
      .commit()
      .accounts({
        user: user_wallet.publicKey
      })
      .signers([user_wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
