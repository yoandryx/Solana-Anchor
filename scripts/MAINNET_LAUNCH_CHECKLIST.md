# LuxHub Mainnet Launch Checklist

## Overview

This document provides a step-by-step guide for deploying LuxHub to Solana mainnet.

---

## Phase 1: Pre-Deployment Preparation

### 1.1 Security Audit
- [ ] Smart contract audit completed by reputable firm
- [ ] All critical/high findings resolved
- [ ] Audit report saved to `/docs/audit/`

### 1.2 Squads Multisig Setup
```
PSEUDO CODE:
1. Go to https://app.squads.so
2. Create new multisig with:
   - Name: "LuxHub Treasury"
   - Threshold: 2-of-3 (or 3-of-5 for higher security)
   - Members: [FOUNDER_WALLET, COFOUNDER_WALLET, BACKUP_WALLET]
3. Note down:
   - Multisig PDA: ________________
   - Vault PDA (index 0): ________________
4. Update deploy-and-init.ts CONFIG.mainnet values
```

### 1.3 Wallet Preparation
- [ ] Create dedicated deployer wallet (not personal wallet)
- [ ] Fund deployer with ~5 SOL for deployment + rent
- [ ] Store deployer keypair securely (hardware wallet backup)
- [ ] Document deployer pubkey: ________________

### 1.4 RPC Configuration
- [ ] Helius mainnet API key obtained
- [ ] Rate limits verified for expected traffic
- [ ] Fallback RPC configured (Alchemy, QuickNode)

---

## Phase 2: Code Freeze & Final Testing

### 2.1 Code Freeze
```bash
# Create release branch
git checkout -b release/v1.0.0-mainnet
git push origin release/v1.0.0-mainnet

# Tag the release
git tag -a v1.0.0-mainnet -m "Mainnet release candidate"
git push origin v1.0.0-mainnet
```

### 2.2 Final Devnet Test
```bash
# Deploy fresh to devnet
cd /home/ycstudio/LuxHub/Solana-Anchor
npx ts-node scripts/deploy-and-init.ts --cluster devnet --action full

# Run full test suite
anchor test

# Manual E2E testing checklist:
# - [ ] Vendor can list NFT (initialize escrow)
# - [ ] Vendor can update price
# - [ ] Vendor can cancel listing
# - [ ] Buyer can purchase (exchange)
# - [ ] Admin can confirm delivery (funds split 97/3)
# - [ ] Verify seller receives 97%
# - [ ] Verify treasury receives 3%
```

### 2.3 Constants Verification
```bash
# Verify fee split is correct
grep -n "SELLER_BPS\|FEE_BPS" programs/luxhub-marketplace/src/constants.rs
# Expected: SELLER_BPS = 9700, FEE_BPS = 300
```

---

## Phase 3: Mainnet Deployment

### 3.1 Update Configuration
```typescript
// In scripts/deploy-and-init.ts, update:
mainnet: {
  squadsMultisig: "YOUR_MAINNET_MULTISIG_PDA",
  squadsVaultPda: "YOUR_MAINNET_VAULT_PDA",
  rpcUrl: "https://mainnet.helius-rpc.com/?api-key=YOUR_MAINNET_KEY",
}
```

### 3.2 Update Anchor.toml
```toml
# Change from devnet to mainnet
[provider]
cluster = "mainnet"
wallet = "~/path/to/mainnet-deployer.json"

[programs.mainnet]
luxhub_marketplace = "TO_BE_FILLED_AFTER_DEPLOY"
```

### 3.3 Deploy
```bash
# Build and deploy
npx ts-node scripts/deploy-and-init.ts --cluster mainnet --action full

# Record the program ID: ________________
```

### 3.4 Verify Deployment
```bash
# Check program on explorer
# https://explorer.solana.com/address/YOUR_PROGRAM_ID

# Verify config was initialized
solana account YOUR_CONFIG_PDA --url mainnet-beta
```

---

## Phase 4: Frontend Configuration

### 4.1 Environment Variables
```bash
# Update .env.production (or Vercel environment)
PROGRAM_ID=YOUR_MAINNET_PROGRAM_ID
NEXT_PUBLIC_SOLANA_ENDPOINT=https://mainnet.helius-rpc.com/?api-key=XXX
NEXT_PUBLIC_SQUADS_MSIG=YOUR_MAINNET_MULTISIG_PDA
NEXT_PUBLIC_VAULT_PDA=YOUR_MAINNET_VAULT_PDA
```

### 4.2 IDL Update
```bash
# Copy mainnet IDL to frontend
cp target/idl/luxhub_marketplace.json ../src/idl/
```

### 4.3 MongoDB VaultConfig
```javascript
// Update VaultConfig in MongoDB for mainnet
db.vaultconfigs.updateOne(
  { _id: ObjectId("YOUR_VAULT_CONFIG_ID") },
  {
    $set: {
      programId: "YOUR_MAINNET_PROGRAM_ID",
      escrowConfigPda: "YOUR_MAINNET_CONFIG_PDA",
      treasuryWallet: "YOUR_MAINNET_VAULT_PDA",
      network: "mainnet-beta"
    }
  }
)
```

---

## Phase 5: Post-Deployment Verification

### 5.1 Smoke Tests
- [ ] Admin dashboard loads with correct program ID
- [ ] Can view escrow config on-chain
- [ ] Test transaction with small amount (if possible)

### 5.2 Monitoring Setup
- [ ] Set up Helius webhooks for program events
- [ ] Configure error alerting (Sentry, etc.)
- [ ] Set up on-chain monitoring (Solana FM, etc.)

### 5.3 Documentation Update
- [ ] Update CLAUDE.md with mainnet program ID
- [ ] Update README with mainnet deployment info
- [ ] Archive devnet program ID for reference

---

## Emergency Procedures

### If Something Goes Wrong

```
PSEUDO CODE - EMERGENCY PAUSE:
1. Do NOT panic
2. If funds at risk:
   a. Contact Squads support immediately
   b. Freeze all multisig operations
   c. Document the issue
3. If UI bug:
   a. Revert Vercel deployment to last known good
   b. Put up maintenance page
4. If smart contract bug:
   a. Halt all admin operations
   b. Assess if upgrade is possible
   c. If critical, coordinate with security researchers
```

### Contact List
- Squads Support: https://discord.gg/squads
- Helius Support: https://discord.gg/helius
- Solana Foundation: security@solana.org

---

## Rollback Plan

```bash
# If mainnet deployment fails, revert to devnet config:

# 1. Revert .env.local
git checkout HEAD -- .env.local

# 2. Revert Anchor.toml
git checkout HEAD -- Anchor.toml

# 3. Redeploy frontend with devnet config
vercel --prod --force
```

---

## Program ID Registry

| Network | Program ID | Config PDA | Deploy Date | Notes |
|---------|------------|------------|-------------|-------|
| Devnet  | kW2w2pHhAP8hFGRLganziunchKu6tjaXyomvF6jxNpj | TBD | Jan 2025 | Current |
| Mainnet | TBD | TBD | TBD | Pending |

---

## Signatures

**Deployment Approved By:**
- [ ] Technical Lead: ________________ Date: ________
- [ ] Security Review: ________________ Date: ________
- [ ] Business Approval: ________________ Date: ________
