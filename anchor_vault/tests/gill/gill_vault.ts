import { createSolanaClient, KeyPairSigner } from "gill";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { loadKeypairSignerFromFile } from "gill/node";
import { getAddMemoInstruction } from "gill/programs";
import { AnchorVault } from "../../target/types/anchor_vault";

import path from "path";
import { should } from "chai";
should();

const { rpc } = createSolanaClient({ urlOrMoniker: "devnet" });

describe("anchor_vault_gill", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;

  let wallet_1: KeyPairSigner;
  let wallet_2: KeyPairSigner;

  beforeEach("setup", async () => {
    wallet_1 = await loadKeypairSignerFromFile(); // loading wallet from Solana CLI's default keypair file (`~/.config/solana/id.json`)

    wallet_2 = await loadKeypairSignerFromFile(
      path.join(__dirname, "../wallets/user-2-wallet.json")
    ); // loads wallet from ../wallets/user-2.json
  });

  it("should load wallets successfully", async () => {
    // some checks to ensure we have a valid address
    wallet_2.address.should.have
      .a("string")
      .with.length.greaterThan(1, "could not load wallet file -  file error");
    wallet_1.address.should.have
      .a("string")
      .with.length.greaterThan(1, "could not load wallet file -  file error");

    console.log("Wallet 1:", wallet_1);
    console.log("Wallet 2:", wallet_2);
  });

  it("Is initialized!", async () => {
    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          payer: wallet_1.address,
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
