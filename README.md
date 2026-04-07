# LuxHub Marketplace — Solana Escrow Program

On-chain escrow protocol for NFT-backed luxury asset transactions on Solana. Physical goods (watches, jewelry, collectibles) are represented as NFTs and held in PDA vaults until delivery is confirmed, then funds are automatically split between vendor and treasury.

**Program ID:** `kW2w2pHhAP8hFGRLganziunchKu6tjaXyomvF6jxNpj`
**Cluster:** Mainnet / Devnet
**Framework:** Anchor 0.31.0 | Solana 2.1.16

## How It Works

```
Vendor lists luxury asset
    -> Admin mints NFT (SPL Token + Metadata)
    -> Admin initializes escrow (NFT locked in PDA vault)

Buyer purchases
    -> Buyer deposits USDC into escrow vault
    -> NFT remains locked until delivery confirmed

Delivery confirmed (Squads multisig required)
    -> NFT transferred to buyer
    -> Funds split: 97% to vendor, 3% to treasury
    -> Vault accounts closed, rent returned

Dispute / Refund (Squads multisig required)
    -> Funds returned to buyer
    -> NFT returned to vendor
    -> Escrow marked complete
```

Every fund-releasing instruction (`confirm_delivery`, `refund_buyer`) is gated by [Squads Protocol v4](https://squads.so/) CPI verification — no single key can move funds.

## Architecture

### Accounts

**EscrowConfig** (singleton PDA) — Protocol-level settings:
- `authority` — Squads multisig that controls the protocol
- `treasury` — Vault PDA where fees are collected
- `fee_bps` — Fee in basis points (300 = 3%, max 1000 = 10%)
- `paused` — Emergency kill switch

**Escrow** (per-listing PDA) — Individual escrow state:
- Seller, buyer, NFT mint, funds mint, sale price
- IPFS CID for off-chain metadata
- Completion status
- PDA-derived ATA vaults for NFT and funds

### Instructions

| Instruction | Who | What |
|---|---|---|
| `initialize_config` | Admin | Create protocol config (authority, treasury, fee) |
| `update_config` | Authority | Update config fields (all optional) |
| `close_config` | Authority | Close config for migration |
| `initialize` | Admin + Seller | Lock NFT in escrow vault, set sale price |
| `exchange` | Buyer | Deposit funds (USDC) into escrow vault |
| `confirm_delivery` | Squads multisig | Release NFT to buyer, split funds 97/3 |
| `refund_buyer` | Squads multisig | Return funds to buyer, NFT to seller |
| `update_price` | Seller | Change listing price (before buyer deposits) |
| `cancel_escrow` | Seller | Cancel listing, reclaim NFT (before buyer deposits) |

### Security

- **Squads CPI gate** — `confirm_delivery` and `refund_buyer` verify the calling instruction originates from Squads v4 (`SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf`). No single wallet can release or refund funds.
- **PDA vaults** — NFT and funds held in escrow-derived ATAs. No arbitrary keypair vaults.
- **Overflow protection** — All fee math uses `checked_mul` / `checked_sub` / `checked_div`.
- **Emergency pause** — Config `paused` flag blocks all fund movements.
- **Fee cap** — `fee_bps` max 1000 (10%) enforced on-chain.
- **Self-purchase prevention** — Buyer cannot be the seller.
- **State guards** — Cannot cancel after buyer deposits, cannot confirm before buyer deposits, cannot exchange after completion.

### Fee Split

```
sale_price = 1,000,000 (1 USDC)
fee_bps    = 300 (3%)

fee_share    = sale_price * 300 / 10,000 = 30,000
seller_share = sale_price - fee_share    = 970,000

-> 970,000 to seller ATA
-> 30,000  to treasury ATA
```

`seller_share = sale_price - fee_share` ensures zero remainder loss from integer division.

## Project Structure

```
programs/luxhub-marketplace/src/
  lib.rs                    # Account contexts + instruction dispatch
  constants.rs              # Seeds, Squads program ID, BPS denominator
  errors.rs                 # Custom error codes
  state/
    config.rs               # EscrowConfig account
    escrow.rs               # Escrow account
  instructions/
    initialize_config.rs    # Create protocol config
    update_config.rs        # Update config fields
    close_config.rs         # Close config (migration)
    initialize.rs           # Create escrow, lock NFT
    exchange.rs             # Buyer deposits funds
    confirm_delivery.rs     # Multisig release: NFT->buyer, funds->seller+treasury
    refund_buyer.rs         # Multisig refund: funds->buyer, NFT->seller
    update_price.rs         # Seller updates listing price
    cancel_escrow.rs        # Seller cancels listing
  utils/
    squads_gate.rs          # Squads v4 CPI origin verification
```

## Build & Test

```bash
# Install dependencies
yarn

# Build
anchor build

# Run tests
anchor test

# Deploy (requires funded wallet)
anchor deploy --provider.cluster devnet
```

## Integration

The program is consumed by the [LuxHub web platform](https://luxhub.gold) ([source](https://github.com/yoandryx/LuxHub)) which provides:

- Vendor onboarding and NFT minting (SPL Token + Irys metadata)
- Admin dashboard for escrow management
- Squads multisig proposal creation and execution
- Buyer purchase flow with on-chain transaction verification
- Dispute system with 7-day SLA and automatic timeout enforcement
- Token pools via Bags API for community-funded listings

## License

See [LICENSE](./LICENSE).
