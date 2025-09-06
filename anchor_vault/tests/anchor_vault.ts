import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import wallet_1_file from "./wallets/user-1-wallet.json";
import wallet_2_file from "./wallets/user-2-wallet.json";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
// import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

const wallet_1 = anchor.web3.Keypair.fromSecretKey(
  new Uint8Array(wallet_1_file)
);
const wallet_2 = anchor.web3.Keypair.fromSecretKey(
  new Uint8Array(wallet_2_file)
);

let tokenMint = new PublicKey("2o39Cm7hzaXmm9zGGGsa5ZiveJ93oMC2D6U7wfsREcCo");

describe("anchor_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;

  it.skip("Is initialized!", async () => {
    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          payer: wallet_1.publicKey,
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
          payer: wallet_1.publicKey,
          tokenMint: null,
          tokenProgram: TOKEN_PROGRAM_ID,
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
    try {
      const tx = await program.methods
        .withdraw(new anchor.BN(1_900_000))
        .accounts({
          payer: wallet_1.publicKey,
          tokenMint,
          tokenProgram: TOKEN_PROGRAM_ID,
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

  it("should deposit spl token to vault", async () => {
    try {
      const tx = await program.methods
        .depositSpl(new anchor.BN(10_000_000)) // 10 tokens - 6 decimals
        .accounts({
          payer: wallet_1.publicKey,
          tokenMint,
          tokenProgram: TOKEN_PROGRAM_ID,
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

  it.only("should withdraw spl token from vault to any wallet", async () => {
    // Add your test here.
    try {
      // let [vaultState] = PublicKey.findProgramAddressSync(
      //   [Buffer.from("vault_state"), wallet_1.publicKey.toBuffer()],
      //   program.programId
      // );

      // let vaultAta = await getOrCreateAssociatedTokenAccount(
      //   // @ts-ignore
      //   program.provider.connection,
      //   wallet_1,
      //   tokenMint,
      //   vaultState,
      //   true
      // );

      // let userAta = await getOrCreateAssociatedTokenAccount(
      //   // @ts-ignore
      //   program.provider.connection,
      //   wallet_1,
      //   tokenMint,
      //   wallet_1.publicKey
      // );
      // console.log("vaultAta: ", vaultAta.address.toBase58());
      // console.log("userAta: ", userAta.address.toBase58());

      const tx = await program.methods
        .withdrawSpl(new anchor.BN(10_000_000)) // 10 tokens - 6 decimals
        .accounts({
          payer: wallet_1.publicKey,
          tokenMint,
          tokenProgram: TOKEN_PROGRAM_ID,
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
