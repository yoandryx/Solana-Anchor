import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { expect } from "chai";

// const { LuxhubMarketplace } = require("../target/types/luxhub_marketplace");

const CONFIG_SEED = Buffer.from("luxhub-config");
const ESCROW_SEED = Buffer.from("state");

describe("luxhub_marketplace", () => {
  // Use env so the cluster matches how you run tests:
  // - `anchor test` -> local validator (localnet)
  // - `anchor test --skip-local-validator` + devnet cfg -> devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.LuxhubMarketplace as anchor.Program;

  // Actors
  const admin = (provider.wallet as anchor.Wallet).payer;
  const seller = Keypair.generate();
  const buyer = Keypair.generate();

  // Token mints
  let mintA: PublicKey; // funds mint (9 decimals)
  let mintB: PublicKey; // NFT mint (0 decimals)

  // Seller token accounts
  let sellerAtaA: PublicKey;
  let sellerAtaB: PublicKey;

  // Buyer token accounts
  let buyerAtaA: PublicKey;
  let buyerAtaB: PublicKey;

  // Config PDA
  let configPda: PublicKey;
  let configBump: number;

  // Escrow PDA from [b"state", seed_le]
  const escrowSeed = new BN(42);
  let escrowPda: PublicKey;
  let escrowBump: number;

  // Vaults (plain TokenAccounts) initialized by the program in `initialize`
  const nftVault = Keypair.generate();
  const wsolVault = Keypair.generate();

  // Sale price (1 token with 9 decimals)
  const salePrice = new BN(1_000_000_000);

  /** ---------- Helpers ---------- */
  const rpc =
    // @ts-ignore
    provider.connection.rpcEndpoint ||
    // @ts-ignore
    (provider.connection as any)._rpcEndpoint ||
    "<unknown>";

  const logHdr = (t: string) => console.log(`\n========== ${t} ==========`);

  async function airdrop(pubkey: PublicKey, sol = 2, label?: string) {
    const sig = await provider.connection.requestAirdrop(
      pubkey,
      sol * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig, "confirmed");
    console.log(`Airdropped ${sol} SOL to ${label ?? pubkey.toBase58()} — tx: ${sig}`);
  }

  async function dumpTokenAcc(label: string, address: PublicKey) {
    try {
      const acc = await getAccount(provider.connection, address);
      console.log(
        `${label}: { addr: ${address.toBase58()}, mint: ${acc.mint.toBase58()}, owner: ${acc.owner.toBase58()}, amount: ${Number(
          acc.amount
        )} }`
      );
    } catch (e) {
      console.log(`${label}: <unavailable> (${address.toBase58()})`, e instanceof Error ? e.message : e);
    }
  }

  async function dumpEscrowState(label: string, escrowPk: PublicKey) {
    try {
      const esc = await (program.account as any)["escrow"].fetch(escrowPk);
      console.log(`${label}:`, {
        seed: esc.seed?.toString?.() ?? esc.seed,
        bump: esc.bump,
        initializer: esc.initializer?.toBase58?.(),
        luxhub_wallet: esc.luxhubWallet?.toBase58?.(),
        mintA: esc.mintA?.toBase58?.(),
        mintB: esc.mintB?.toBase58?.(),
        initializerAmount: Number(esc.initializerAmount),
        takerAmount: Number(esc.takerAmount),
        fileCid: esc.fileCid,
        salePrice: Number(esc.salePrice),
        isCompleted: esc.isCompleted,
        buyer: esc.buyer?.toBase58?.(),
      });
    } catch (e) {
      console.log(`${label}: <failed to fetch escrow>`, e);
    }
  }

  it("environment info", async () => {
    logHdr("ENV");
    console.log("Program ID:", program.programId.toBase58());
    console.log("Cluster RPC:", rpc);
    console.log("Admin (payer):", admin.publicKey.toBase58());
    console.log("Seller:", seller.publicKey.toBase58());
    console.log("Buyer :", buyer.publicKey.toBase58());
  });

  it("sets up wallets, mints, and accounts", async () => {
    logHdr("SETUP");

    await airdrop(seller.publicKey, 2, "seller");
    await airdrop(buyer.publicKey, 2, "buyer");

    // Create funds mint (decimals 9)
    mintA = await createMint(provider.connection, admin, admin.publicKey, null, 9);
    console.log("Created mintA:", mintA.toBase58());

    // Create NFT mint (decimals 0)
    mintB = await createMint(provider.connection, admin, admin.publicKey, null, 0);
    console.log("Created mintB (NFT):", mintB.toBase58());

    // Seller ATAs
    sellerAtaA = (
      await getOrCreateAssociatedTokenAccount(provider.connection, admin, mintA, seller.publicKey)
    ).address;
    console.log("sellerAtaA:", sellerAtaA.toBase58());

    sellerAtaB = (
      await getOrCreateAssociatedTokenAccount(provider.connection, admin, mintB, seller.publicKey)
    ).address;
    console.log("sellerAtaB (NFT):", sellerAtaB.toBase58());

    // Mint 1 NFT to seller
    const mintNftSig = await mintTo(provider.connection, admin, mintB, sellerAtaB, admin.publicKey, 1);
    console.log("Minted 1 NFT to sellerAtaB — tx:", mintNftSig);

    // Buyer ATAs
    buyerAtaA = (
      await getOrCreateAssociatedTokenAccount(provider.connection, admin, mintA, buyer.publicKey)
    ).address;
    console.log("buyerAtaA:", buyerAtaA.toBase58());

    buyerAtaB = (
      await getOrCreateAssociatedTokenAccount(provider.connection, admin, mintB, buyer.publicKey)
    ).address;
    console.log("buyerAtaB:", buyerAtaB.toBase58());

    // Fund buyer with mintA
    const mintFundsSig = await mintTo(
      provider.connection,
      admin,
      mintA,
      buyerAtaA,
      admin.publicKey,
      salePrice.toNumber()
    );
    console.log(`Minted ${salePrice.toString()} units to buyerAtaA — tx:`, mintFundsSig);

    // Derive config PDA
    [configPda, configBump] = PublicKey.findProgramAddressSync([CONFIG_SEED], program.programId);

    // Derive escrow PDA from seed (u64 LE)
    const seedLE = Buffer.alloc(8);
    seedLE.writeBigUInt64LE(BigInt(escrowSeed.toString()));
    [escrowPda, escrowBump] = PublicKey.findProgramAddressSync([ESCROW_SEED, seedLE], program.programId);

    console.log("configPda:", configPda.toBase58(), "bump", configBump);
    console.log("escrowPda:", escrowPda.toBase58(), "bump", escrowBump);

    console.log("Initial token balances:");
    await dumpTokenAcc("sellerAtaA", sellerAtaA);
    await dumpTokenAcc("sellerAtaB", sellerAtaB);
    await dumpTokenAcc("buyerAtaA", buyerAtaA);
    await dumpTokenAcc("buyerAtaB", buyerAtaB);
  });

  it("initialize_config", async () => {
    logHdr("INITIALIZE_CONFIG");

    const squadsMultisig = Keypair.generate().publicKey;
    const squadsAuthority = Keypair.generate().publicKey;

    try {
      const sig = await program.methods
        .initializeConfig(squadsMultisig, squadsAuthority)
        .accounts({
          admin: admin.publicKey,
          config: configPda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("initialize_config tx:", sig);
    } catch (e: any) {
      console.error("initialize_config failed:", e?.logs ?? e);
      throw e;
    }

    const cfg = await (program.account as any)["escrowConfig"].fetch(configPda);
    console.log("escrowConfig fetched:", {
      squadsMultisig: cfg.squadsMultisig.toBase58(),
      squadsAuthority: cfg.squadsAuthority.toBase58(),
    });

    expect(cfg.squadsMultisig.toBase58()).to.eq(squadsMultisig.toBase58());
    expect(cfg.squadsAuthority.toBase58()).to.eq(squadsAuthority.toBase58());
  });

  it("initialize escrow (moves seller NFT -> nft_vault)", async () => {
    logHdr("INITIALIZE");

    const initializerAmount = new BN(1);
    const takerAmount = new BN(0);
    const fileCid = "ipfs://demo-cid";

    console.log("Pre-initialize balances:");
    await dumpTokenAcc("sellerAtaB", sellerAtaB);

    try {
      const sig = await program.methods
        .initialize(escrowSeed, initializerAmount, takerAmount, fileCid, salePrice)
        .accounts({
          admin: admin.publicKey,
          seller: seller.publicKey,
          config: configPda,
          mintA,
          mintB,
          sellerAtaA,
          sellerAtaB,
          escrow: escrowPda,
          nftVault: nftVault.publicKey, // created via #[account(init)]
          wsolVault: wsolVault.publicKey, // created via #[account(init)]
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([seller, nftVault, wsolVault])
        .rpc();

      console.log("initialize tx:", sig);
    } catch (e: any) {
      console.error("initialize failed:", e?.logs ?? e);
      throw e;
    }

    console.log("Post-initialize balances:");
    await dumpTokenAcc("sellerAtaB", sellerAtaB);
    await dumpTokenAcc("nftVault", nftVault.publicKey);
    await dumpTokenAcc("wsolVault", wsolVault.publicKey);

    const sellerNftAcc = await getAccount(provider.connection, sellerAtaB);
    const vaultNftAcc = await getAccount(provider.connection, nftVault.publicKey);
    expect(Number(sellerNftAcc.amount)).to.eq(0);
    expect(Number(vaultNftAcc.amount)).to.eq(1);

    await dumpEscrowState("Escrow state after initialize", escrowPda);
    const escrowAcc = await (program.account as any)["escrow"].fetch(escrowPda);
    expect(escrowAcc.initializer.toBase58()).to.eq(seller.publicKey.toBase58());
    expect(escrowAcc.mintA.toBase58()).to.eq(mintA.toBase58());
    expect(escrowAcc.mintB.toBase58()).to.eq(mintB.toBase58());
    expect(Number(escrowAcc.initializerAmount)).to.eq(1);
    expect(escrowAcc.isCompleted).to.eq(false);
  });

  it("exchange (buyer deposits funds -> wsol_vault)", async () => {
    logHdr("EXCHANGE");

    console.log("Pre-exchange balances:");
    await dumpTokenAcc("buyerAtaA", buyerAtaA);
    await dumpTokenAcc("wsolVault", wsolVault.publicKey);

    try {
      const sig = await program.methods
        .exchange()
        .accounts({
          taker: buyer.publicKey,
          escrow: escrowPda,
          mintA,
          mintB,
          takerFundsAta: buyerAtaA,
          takerNftAta: buyerAtaB,
          // IMPORTANT: the account name must match your Rust field.
          // If your Exchange struct uses `wsol_vault`, this must be `wsolVault`.
          wsolVault: wsolVault.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([buyer])
        .rpc();

      console.log("exchange tx:", sig);
    } catch (e: any) {
      console.error("exchange failed:", e?.logs ?? e);
      throw e;
    }

    console.log("Post-exchange balances:");
    await dumpTokenAcc("buyerAtaA", buyerAtaA);
    await dumpTokenAcc("wsolVault", wsolVault.publicKey);

    const buyerFundsAfter = await getAccount(provider.connection, buyerAtaA);
    const wsolVaultAcc = await getAccount(provider.connection, wsolVault.publicKey);

    expect(Number(buyerFundsAfter.amount)).to.eq(0);
    expect(Number(wsolVaultAcc.amount)).to.eq(salePrice.toNumber());
  });

  // confirm_delivery & admin_only_example remain gated by Squads CPI.
});
