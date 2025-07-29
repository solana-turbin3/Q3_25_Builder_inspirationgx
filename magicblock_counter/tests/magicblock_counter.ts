import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MagicblockCounter } from "../target/types/magicblock_counter";
import user_file from "./wallets/user-1-wallet.json";
import user_file_2 from "./wallets/user-2-wallet.json";
import user_file_3 from "./wallets/user-3-wallet.json";
import { expect } from "chai";

const user_wallet = anchor.web3.Keypair.fromSecretKey(
  new Uint8Array(user_file_3)
);

describe("magicblock_counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .magicblockCounter as Program<MagicblockCounter>;

  const getCounterPDA = () => {
    const [counter] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("COUNT"), user_wallet.publicKey.toBuffer()],
      program.programId
    );
    return counter;
  };

  // Reset test environment before running tests
  before("Setup test environment", async () => {
    try {
      const counterPDA = getCounterPDA();
      const provider = anchor.getProvider();

      // Check if counter account exists and try to close it
      const accountInfo = await provider.connection.getAccountInfo(counterPDA);
      if (accountInfo) {
        console.log(
          "Counter account exists, current owner:",
          accountInfo.owner.toString()
        );
        console.log("Expected owner:", program.programId.toString());

        // If it's owned by delegation program, we need to undelegate first
        if (accountInfo.owner.toString() !== program.programId.toString()) {
          console.log("Account is delegated, attempting to undelegate...");
          try {
            await program.methods
              .undelegate()
              .accounts({
                user: user_wallet.publicKey
              })
              .signers([user_wallet])
              .rpc();
            console.log("Successfully undelegated account");
          } catch (error) {
            console.log("Could not undelegate:", error.message);
            console.log(
              "You may need to restart your validator or use a different test wallet"
            );
          }
        }
      }
    } catch (error) {
      console.log("Setup error (this may be expected):", error.message);
    }
  });

  it("Is initialized!", async () => {
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
    // This should fail because the account is now delegated
    try {
      const tx = await program.methods
        .increment()
        .accounts({
          user: user_wallet.publicKey
        })
        .signers([user_wallet])
        .rpc();

      // If we reach here, the test should fail
      expect.fail("Expected transaction to fail but it succeeded");
    } catch (error) {
      // Verify it's the expected AccountOwnedByWrongProgram error
      expect(error.error.errorCode.code).to.equal("AccountOwnedByWrongProgram");
      console.log("Expected error occurred:", error.error.errorMessage);
    }
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
      .incrementAndCommit()
      .accounts({
        user: user_wallet.publicKey
      })
      .signers([user_wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("should undelegate back to program", async () => {
    // Add your test here.
    const tx = await program.methods
      .undelegate()
      .accounts({
        user: user_wallet.publicKey
      })
      .signers([user_wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("should increment normally after undelegation", async () => {
    const tx = await program.methods
      .increment()
      .accounts({
        user: user_wallet.publicKey
      })
      .signers([user_wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
