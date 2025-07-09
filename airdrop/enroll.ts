import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, type Turbin3Prereq } from "./programs/Turbin3_prereq.ts";
import wallet from "./Turbin3-wallet.json" with { type: 'json' };
import mintNFT from "./mintNFT-wallet.json" with { type: 'json' };

const MPL_CORE_PROGRAM_ID = new PublicKey(
  "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
);
const mintCollection = new PublicKey(
  "5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2"
);

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");

const mintTs = Keypair.fromSecretKey(Uint8Array.from(mintNFT))

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed"
});

// Create our program
// @ts-expect-error
const program: Program<Turbin3Prereq> = new Program(IDL, provider);

// Create the PDA for our enrollment account
const account_seeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()];
const [account_key, _account_bump] = PublicKey.findProgramAddressSync(
  account_seeds,
  program.programId
);

// (async () => {
//   try {
//     const txhash = await program.methods
//       .initialize("inspi-writer001")
//       .accountsPartial({
//         user: keypair.publicKey,
//         account: account_key,
//         systemProgram: SystemProgram.programId
//       })
//       .signers([keypair])
//       .rpc();
//     console.log(`Success! Check out your TX here:
//     https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
//   } catch (e) {
//     console.error(`Oops, something went wrong: ${e}`);
//   }
// })();

// Execute the submitTs transaction
(async () => {
  try {
    const txhash = await program.methods

      .submitTs()
      .accountsPartial({
        user: keypair.publicKey,
        account: account_key,
        mint: mintTs.publicKey,
        collection: mintCollection,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
        systemProgram:  SystemProgram.programId
      })
      .signers([keypair, mintTs])
      .rpc();
    console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
