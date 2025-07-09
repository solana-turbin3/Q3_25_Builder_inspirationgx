# 🔥 Turbin3 Prereqs and starter

_This Directory contains submission demonstrating Solana blockchain expertise through Anchor programs -[ Vaults, escrows, etc ], PDAs, and token operations_

![Turbine Banner](https://pbs.twimg.com/profile_banners/1707159181914976256/1748632505/1500x500)

## 📂 Project Structure

```bash
.
├── airdrop/                # Token distribution system
│   ├── programs/           # IDL + typed clients
│   ├── Turbin3_prereq_idl/ # Program interfaces
│   └── *.ts                # Operational scripts
├── anchor_vault/           # Secure asset vault
│   ├── programs/           # On-chain logic
│   └── tests/              # Integration tests
├── solana-starter/         # Core environment - where it all began
│   ├── rs/                 # Rust programs
│   └── ts/                 # Client operations that might interest you 🤝
└── test-class-1/           # WIP
...
```

## 🏗 Core Components

### 1. Airdrop System (TypeScript) - code snipppets

```typescript
// airdrop/enroll.ts
const [pda, bump] = await PublicKey.findProgramAddress(
  [Buffer.from("enroll"), user.key.toBuffer()],
  programId
);

await program.methods
  .enroll(bump)
  .accounts({ user: user.publicKey, pda })
  .rpc();
```

- PDA-derived enrollment system
- Supports SPL tokens and NFTs
- Automated eligibility checks

### 2. Anchor Vault (Rust) - code snipppets

```rust
// programs/anchor_vault/src/lib.rs
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [b"vault", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
    // ...
}

#[error_code]
pub enum VaultError {
    #[msg("Withdrawal timelock not expired")]
    TimelockActive,
}
```

- Time-locked withdrawals
- PDA-secured storage
- Custom error codes

## 🚀 Deployment

in Anchor project dirrectories, you should run:

```bash
# 1. Build programs
anchor build

# 2. Deploy to Envirronment specified in Anchor.toml
anchor deploy

# 3. Run tests
anchor test --skip-build --skip-deploy
```

## 📬 Contact

**For verification or opportunities:**  
✉️ [conservedinnovation@gmail.com](mailto:your.email@example.com)  
🐦 [@inspiration_gx](https://twitter.com/yourhandle)

_"This submission represents my own work in accordance with academic integrity policies."_

- **[InspirationGx]**  
  _Solana Developer & Student_
