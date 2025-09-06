import { useState } from "react";
import {
  useConnection,
  useWallet,
  useAnchorWallet,
} from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Program, AnchorProvider, BN, AnchorError } from "@coral-xyz/anchor";
import IDL from "../program/idl/anchor_vault.json";
import type { AnchorVault } from "../program/types/anchor_vault.ts";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

const SPL_TOKEN_MINT = new PublicKey(
  "2o39Cm7hzaXmm9zGGGsa5ZiveJ93oMC2D6U7wfsREcCo" // you can use your own token mint here
);

export default function VaultInterface() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const wallet = useAnchorWallet();

  const [solAmount, setSolAmount] = useState("");
  const [splAmount, setSplAmount] = useState("");
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState("");

  const getProvider = () => {
    if (!wallet || !publicKey) {
      alert("could not find your solana wallet");
      return null;
    }
    return new AnchorProvider(connection, wallet, {});
  };

  const depositSol = async () => {
    if (!publicKey || !solAmount) return;

    setLoading(true);
    setStatus("Depositing SOL...");

    try {
      const provider = getProvider();
      if (!provider) throw new Error("Wallet not connected");

      const program = new Program<AnchorVault>(IDL, provider);

      const tx = await program.methods
        .deposit(new BN(parseFloat(solAmount) * LAMPORTS_PER_SOL))
        .accounts({
          payer: publicKey,
          tokenMint: SPL_TOKEN_MINT,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([])
        .rpc();

      setStatus(`✅ SOL deposited! TX: ${tx.slice(0, 8)}...`);
      setSolAmount("");
    } catch (error) {
      let err = error as AnchorError;
      console.error("Deposit error:", error);
      setStatus(`❌ Deposit failed: ${err.message}`);
    }

    setLoading(false);
  };

  const withdrawSol = async () => {
    if (!publicKey || !solAmount) return;

    setLoading(true);
    setStatus("Withdrawing SOL...");

    try {
      const provider = getProvider();
      if (!provider) throw new Error("Wallet not connected");

      const program = new Program<AnchorVault>(IDL, provider);

      const tx = await program.methods
        .withdraw(new BN(parseFloat(solAmount) * LAMPORTS_PER_SOL))
        .accounts({
          payer: publicKey,
          tokenMint: SPL_TOKEN_MINT,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();

      setStatus(`✅ SOL withdrawn! TX: ${tx.slice(0, 8)}...`);
      setSolAmount("");
    } catch (error) {
      let err = error as AnchorError;
      console.error("Deposit error:", error);
      setStatus(`❌ Deposit failed: ${err.message}`);
    }

    setLoading(false);
  };

  const depositSpl = async () => {
    if (!publicKey || !splAmount) return;

    setLoading(true);
    setStatus("Depositing SPL tokens...");

    try {
      const provider = getProvider();
      if (!provider) throw new Error("Wallet not connected");

      const program = new Program<AnchorVault>(IDL, provider);

      const tx = await program.methods
        .depositSpl(new BN(parseFloat(splAmount) * 1_000_000)) // 6 decimals
        .accounts({
          payer: publicKey,
          tokenMint: SPL_TOKEN_MINT,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();

      setStatus(`✅ SPL tokens deposited! TX: ${tx.slice(0, 8)}...`);
      setSplAmount("");
    } catch (error) {
      let err = error as AnchorError;
      console.error("Deposit error:", error);
      setStatus(`❌ Deposit failed: ${err.message}`);
    }

    setLoading(false);
  };

  const withdrawSpl = async () => {
    if (!publicKey || !splAmount) return;

    setLoading(true);
    setStatus("Withdrawing SPL tokens...");

    try {
      const provider = getProvider();
      if (!provider) throw new Error("Wallet not connected");

      const program = new Program<AnchorVault>(IDL, provider);

      const tx = await program.methods
        .withdrawSpl(new BN(parseFloat(splAmount) * 1_000_000)) // 6 decimals
        .accounts({
          payer: publicKey,
          tokenMint: SPL_TOKEN_MINT,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();

      setStatus(`✅ SPL tokens withdrawn! TX: ${tx.slice(0, 8)}...`);
      setSplAmount("");
    } catch (error) {
      let err = error as AnchorError;
      console.error("Deposit error:", error);
      setStatus(`❌ Deposit failed: ${err.message}`);
    }

    setLoading(false);
  };

  return (
    <div className="max-w-md mx-auto p-6 space-y-6 w-full">
      {/* Header */}
      <div className="text-center space-y-2 flex flex-col">
        <h1 className="text-xl font-bold text-white text-nowrap self-center">
          Solana Vault Example
        </h1>
        <p className="text-slate-400 text-sm">
          Deposit & withdraw SOL and SPL tokens
        </p>
      </div>

      {/* Wallet Connection */}
      <div className="flex justify-center">
        <WalletMultiButton className="!bg-slate-700 hover:!bg-slate-600" />
      </div>

      {publicKey && (
        <>
          {/* SOL Operations */}
          <div className="bg-slate-800 rounded-lg p-4 space-y-4">
            <h2 className="text-lg font-semibold text-white">SOL Operations</h2>

            <input
              type="number"
              step="0.001"
              placeholder="Amount in SOL"
              value={solAmount}
              onChange={(e) => setSolAmount(e.target.value)}
              className="w-full p-3 bg-slate-700 text-white rounded-lg border-0 focus:ring-2 focus:ring-slate-500"
            />

            <div className="grid grid-cols-2 gap-3">
              <button
                onClick={depositSol}
                disabled={loading || !solAmount}
                className="bg-emerald-600 hover:bg-emerald-700 disabled:bg-slate-600 text-white font-medium py-3 rounded-lg transition-colors"
              >
                Deposit SOL
              </button>

              <button
                onClick={withdrawSol}
                disabled={loading || !solAmount}
                className="bg-red-600 hover:bg-red-700 disabled:bg-slate-600 text-white font-medium py-3 rounded-lg transition-colors"
              >
                Withdraw SOL
              </button>
            </div>
          </div>

          {/* SPL Token Operations */}
          <div className="bg-slate-800 rounded-lg p-4 space-y-4">
            <h2 className="text-lg font-semibold text-white">
              SPL Token Operations
            </h2>
            <p className="text-xs text-slate-400">Token: 2o39...REcCo</p>

            <input
              type="number"
              step="0.000001"
              placeholder="Amount in tokens"
              value={splAmount}
              onChange={(e) => setSplAmount(e.target.value)}
              className="w-full p-3 bg-slate-700 text-white rounded-lg border-0 focus:ring-2 focus:ring-slate-500"
            />

            <div className="grid grid-cols-2 gap-3">
              <button
                onClick={depositSpl}
                disabled={loading || !splAmount}
                className="bg-blue-600 hover:bg-blue-700 disabled:bg-slate-600 text-white font-medium py-3 rounded-lg transition-colors"
              >
                Deposit SPL
              </button>

              <button
                onClick={withdrawSpl}
                disabled={loading || !splAmount}
                className="bg-purple-600 hover:bg-purple-700 disabled:bg-slate-600 text-white font-medium py-3 rounded-lg transition-colors"
              >
                Withdraw SPL
              </button>
            </div>
          </div>

          {/* Status */}
          {status && (
            <div className="bg-slate-800 rounded-lg p-4">
              <p className="text-white text-sm break-all">{status}</p>
            </div>
          )}

          {/* Loading Indicator */}
          {loading && (
            <div className="text-center">
              <div className="inline-block animate-spin rounded-full h-6 w-6 border-b-2 border-white"></div>
            </div>
          )}
        </>
      )}

      {/* Footer */}
      <div className="text-center text-xs text-slate-500">
        Connected to Devnet
      </div>
    </div>
  );
}
