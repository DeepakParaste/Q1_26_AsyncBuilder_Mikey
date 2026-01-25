# Solana Fungible Token Creation 

Successfully completed all tasks for creating a fungible token on Solana devnet as part of the Turbin3 Q4 2025 Builders Cohort assignment.

## Summary

Created a new SPL token with metadata and transferred tokens to the specified recipient address. All transactions were executed on Solana devnet.

---

## Wallet Setup

**Wallet Address:** `8SBeiD5TwGGhoBuHyQJAq7M2mEzmDjbivuyHyRuoEcXh`

- Generated new keypair using keygen script
- Saved to `turbin3-wallet.json`
- Funded with 2 SOL via devnet airdrop
- **Airdrop Signature:** `2Jasrt8GCNd563s4RrWoU3HcPUFfhRxginfw5YDbpUDGgaBfthgoPpwPpNjeriAjyNmf6D6Na5w4iG17xGtHPyeq`

---

## Task 1: Create Token Mint (spl_init.ts)

**Mint Address:** `H1Ew9Etj2qS8KUC6sedYCgqBN6PSzv4yo2GafhQrusXr`

- Created new SPL token mint with 6 decimals
- Wallet set as mint authority
- [View on Solana Explorer](https://explorer.solana.com/address/H1Ew9Etj2qS8KUC6sedYCgqBN6PSzv4yo2GafhQrusXr?cluster=devnet)

---

## Task 2: Mint Tokens (spl_mint.ts)

**Associated Token Account (ATA):** `43aZXpv8U2zHgbtzQeAjcdxdTfzSqFeXagGc7oqBmpEN`  
**Transaction ID:** `2iFZZsYXwR3ntqpHNWrucgitXEauCt2J6tjdUDi22rKaDJU7JiHWaLzeXFfgS8mdxrcDMoW59W4WNMEcb6tSSj7d`

- Created Associated Token Account for the wallet
- Minted 10,000 tokens (10,000,000,000 base units with 6 decimals)
- [View Transaction](https://explorer.solana.com/tx/2iFZZsYXwR3ntqpHNWrucgitXEauCt2J6tjdUDi22rKaDJU7JiHWaLzeXFfgS8mdxrcDMoW59W4WNMEcb6tSSj7d?cluster=devnet)

---

## Task 3: Add Token Metadata (spl_metadata.ts)

**Transaction Signature:** `2ju7kkgy67Mzi7WJWQb1eknvGyw2XBEPpQpGqh5X118t8F52Fh3bj1ZbgRMMtx4rbs9WqZht4U8UV4U3R86T2Tti`

- Added metadata to the token mint
- Token Name: "CringeToken"
- Token Symbol: "Crngy"
- Seller Fee Basis Points: 500 (5%)
- [View Transaction](https://explorer.solana.com/tx/2ju7kkgy67Mzi7WJWQb1eknvGyw2XBEPpQpGqh5X118t8F52Fh3bj1ZbgRMMtx4rbs9WqZht4U8UV4U3R86T2Tti?cluster=devnet)

---

## Task 4: Transfer Tokens (spl_transfer.ts)

**Recipient Address:** `berg7BKPHZWPiAdjpitQaWCfTELaKjQ6x7e9nDSu23d`  
**Transaction Signature:** `4xiYAfS9TdCJyL9tzjaa3n4qheCwAiUekDqAZNjDZYtG3dQ6iDBo4PqKe594u4nRucbjEqrNJ95FiuxhPZ6dCrA6`

- Transferred 100 tokens (100,000,000 base units with 6 decimals)
- Created recipient's Associated Token Account automatically
- [View Transaction](https://explorer.solana.com/tx/4xiYAfS9TdCJyL9tzjaa3n4qheCwAiUekDqAZNjDZYtG3dQ6iDBo4PqKe594u4nRucbjEqrNJ95FiuxhPZ6dCrA6?cluster=devnet)

---

## Verification

All scripts executed successfully:

 `npm run spl_init` - Mint created  
 `npm run spl_mint` - Tokens minted to ATA  
 `npm run spl_metadata` - Metadata added  
 `npm run spl_transfer` - Tokens transferred to recipient

All transactions can be verified on [Solana Explorer (Devnet)](https://explorer.solana.com/?cluster=devnet).

---
