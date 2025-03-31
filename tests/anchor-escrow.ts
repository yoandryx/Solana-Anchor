import * as anchor from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
} from "@solana/spl-token";
import { randomBytes } from "crypto";
// import BN from "bn.js";
import { BN }from "@coral-xyz/anchor";

describe("anchor-escrow", () => {
  let anchorModule: any;
  let provider: any;
  let connection: any;
  let program: any;

  const [initializer, taker, mintA, mintB] = Array.from({ length: 4 }, () => Keypair.generate());
  let initializerAtaA: PublicKey,
    initializerAtaB: PublicKey,
    takerAtaA: PublicKey,
    takerAtaB: PublicKey;
  let seed: BN;
  let escrow: PublicKey;
  let vault: PublicKey;
  let accounts: any;

  before(async () => {
    anchorModule = await import("@coral-xyz/anchor");
    provider = anchorModule.AnchorProvider.env();
    anchorModule.setProvider(provider);
    connection = provider.connection;
    program = anchorModule.workspace.AnchorEscrow;

    initializerAtaA = getAssociatedTokenAddressSync(mintA.publicKey, initializer.publicKey);
    initializerAtaB = getAssociatedTokenAddressSync(mintB.publicKey, initializer.publicKey);
    takerAtaA = getAssociatedTokenAddressSync(mintA.publicKey, taker.publicKey);
    takerAtaB = getAssociatedTokenAddressSync(mintB.publicKey, taker.publicKey);

    // Use a consistent seed for PDA derivation.
    seed = new BN(Date.now());
    escrow = PublicKey.findProgramAddressSync(
      [Buffer.from("state"), seed.toArrayLike(Buffer, "le", 8)],
      program.programId
    )[0];
    vault = getAssociatedTokenAddressSync(mintA.publicKey, escrow, true);

    accounts = {
      initializer: initializer.publicKey,
      taker: taker.publicKey,
      mintA: mintA.publicKey,
      mintB: mintB.publicKey,
      initializerAtaA,
      initializerAtaB,
      takerAtaA,
      takerAtaB,
      escrow, // computed from the consistent seed
      vault,
      associatedTokenprogram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    };
  });

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({ signature, ...block });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  it("Airdrop and create mints", async () => {
    const providerPublicKey = provider.publicKey;
    if (!providerPublicKey) {
      console.error("Provider's public key is undefined.");
      return;
    }

    const lamports = await getMinimumBalanceForRentExemptMint(connection);
    const tx = new Transaction();
    tx.instructions = [
      ...[initializer, taker].map((k) =>
        SystemProgram.transfer({
          fromPubkey: providerPublicKey,
          toPubkey: k.publicKey,
          lamports: 0.01 * LAMPORTS_PER_SOL,
        })
      ),
      ...[mintA, mintB].map((m) =>
        SystemProgram.createAccount({
          fromPubkey: providerPublicKey,
          newAccountPubkey: m.publicKey,
          lamports,
          space: MINT_SIZE,
          programId: TOKEN_PROGRAM_ID,
        })
      ),
      ...[
        [mintA.publicKey, initializer.publicKey, initializerAtaA],
        [mintB.publicKey, taker.publicKey, takerAtaB],
      ].flatMap((x) => [
        createInitializeMint2Instruction(x[0], 6, x[1], null),
        createAssociatedTokenAccountIdempotentInstruction(providerPublicKey, x[2], x[1], x[0]),
        createMintToInstruction(x[0], x[2], x[1], 1e9),
      ]),
    ];

    await provider.sendAndConfirm(tx, [mintA, mintB, initializer, taker]).then(log);
  });

  it("Initialize", async () => {
    const initializerAmount = 1e6;
    const takerAmount = 1e6;
    const fileCid = "bafkreigyvat7qm4g2bike2bccupazgda6nmbknizjqni42j7nzzrz625ai";

    // Recompute escrow PDA using the same seed and constant "state"
    const [escrowPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("state"), seed.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const initAccounts = { ...accounts, escrow: escrowPDA };

    await program.methods
      .initialize(seed, new BN(initializerAmount), new BN(takerAmount), fileCid)
      .accounts(initAccounts)
      .signers([initializer])
      .rpc();

    // For debugging: fetch and log the escrow account data
    const escrowData = await program.account.escrow.fetch(escrowPDA);
    console.log("Escrow account data:", escrowData);
  });

  xit("Cancel", async () => {
    await program.methods
      .cancel()
      .accounts({ ...accounts })
      .signers([initializer])
      .rpc();
  });

  it("Exchange", async () => {
    await program.methods
      .exchange()
      .accounts({ ...accounts })
      .signers([taker])
      .rpc();
  });
});
