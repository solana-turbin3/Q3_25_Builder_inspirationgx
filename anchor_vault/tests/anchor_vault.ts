import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import wallet_1_file from "./wallets/user-1-wallet.json";
import wallet_2_file from "./wallets/user-2-wallet.json";

const wallet_1 = anchor.web3.Keypair.fromSecretKey(
  new Uint8Array(wallet_1_file)
);
const wallet_2 = anchor.web3.Keypair.fromSecretKey(
  new Uint8Array(wallet_2_file)
);

describe("anchor_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;

  it("Is initialized!", async () => {
    // Add your test here.
    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          payer: wallet_1.publicKey
        })
        .signers([wallet_1])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      if (error.logs) {
        throw error.logs;
      }
      console.log(error);
    }
  });

  it("should deposit to vault", async () => {
    // Add your test here.
    try {
      const tx = await program.methods
        .deposit(new anchor.BN(1_000_000_0))
        .accounts({
          payer: wallet_1.publicKey
        })
        .signers([wallet_1])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      if (error.logs) {
        throw error.logs;
      }
      console.log(error);
    }
  });

  it("should withdraw from vault", async () => {
    // Add your test here.
    try {
      const tx = await program.methods
        .withdraw(new anchor.BN(1_000_000_0))
        .accounts({
          payer: wallet_1.publicKey
        })
        .signers([wallet_1])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      if (error.logs) {
        throw error.logs;
      }
      console.log(error);
    }
  });
});
