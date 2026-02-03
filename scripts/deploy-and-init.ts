/**
 * LuxHub Deployment & Initialization Script
 *
 * Usage:
 *   npx ts-node scripts/deploy-and-init.ts --cluster devnet --action deploy
 *   npx ts-node scripts/deploy-and-init.ts --cluster devnet --action init-config
 *   npx ts-node scripts/deploy-and-init.ts --cluster devnet --action full
 *
 * Actions:
 *   deploy      - Build and deploy the program
 *   init-config - Initialize the escrow config PDA
 *   full        - Deploy + init-config in sequence
 */

import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import * as fs from "fs";
import * as path from "path";
import { execSync } from "child_process";
import { fileURLToPath } from "url";
import { dirname } from "path";

// ESM compatibility
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// ============================================
// CONFIGURATION - UPDATE THESE FOR YOUR SETUP
// ============================================

const CONFIG = {
  // Wallet paths
  deployerWallet: process.env.DEPLOYER_WALLET || "~/LuxHub/Solana-Anchor/keys/dev-wallet.json",

  // Squads Configuration (UPDATE FOR MAINNET)
  devnet: {
    squadsMultisig: "EEtCfR8kJxQ3ZVVtTSkVRXEkF4FfAyt9YnMSiXhtFMLJ",
    squadsVaultPda: "CMJH55q2TFiwaRBna99uMGTEE627avRhLTozfQ6kYksu",
    rpcUrl: process.env.HELIUS_ENDPOINT || "https://devnet.helius-rpc.com/?api-key=351a951c-ee1d-4d8f-87c3-9d1f371cc146",
  },
  mainnet: {
    squadsMultisig: "TODO_MAINNET_MULTISIG",  // <-- UPDATE BEFORE MAINNET
    squadsVaultPda: "TODO_MAINNET_VAULT",     // <-- UPDATE BEFORE MAINNET
    rpcUrl: "https://mainnet.helius-rpc.com/?api-key=YOUR_KEY",
  },

  // Paths
  anchorRoot: path.resolve(__dirname, ".."),
  idlOutput: path.resolve(__dirname, "../../src/idl/luxhub_marketplace.json"),
  envFile: path.resolve(__dirname, "../../.env.local"),
};

// ============================================
// HELPER FUNCTIONS
// ============================================

function loadKeypair(keypairPath: string): Keypair {
  const expanded = keypairPath.replace("~", process.env.HOME || "");
  const secretKey = JSON.parse(fs.readFileSync(expanded, "utf-8"));
  return Keypair.fromSecretKey(Uint8Array.from(secretKey));
}

function getProvider(cluster: "devnet" | "mainnet"): AnchorProvider {
  const rpcUrl = CONFIG[cluster].rpcUrl;
  const connection = new Connection(rpcUrl, "confirmed");
  const wallet = new Wallet(loadKeypair(CONFIG.deployerWallet));
  return new AnchorProvider(connection, wallet, { commitment: "confirmed" });
}

function runCommand(cmd: string, cwd?: string): string {
  console.log(`\n> ${cmd}`);
  try {
    return execSync(cmd, {
      cwd: cwd || CONFIG.anchorRoot,
      encoding: "utf-8",
      stdio: "inherit"
    }) || "";
  } catch (e: any) {
    throw new Error(`Command failed: ${cmd}\n${e.message}`);
  }
}

function updateEnvFile(programId: string): void {
  const envPath = CONFIG.envFile;
  if (!fs.existsSync(envPath)) {
    console.log("⚠️  .env.local not found, skipping env update");
    return;
  }

  let content = fs.readFileSync(envPath, "utf-8");

  // Update PROGRAM_ID
  if (content.includes("PROGRAM_ID=")) {
    content = content.replace(/PROGRAM_ID=.*/, `PROGRAM_ID=${programId}`);
  } else {
    content += `\nPROGRAM_ID=${programId}`;
  }

  fs.writeFileSync(envPath, content);
  console.log(`✅ Updated .env.local with PROGRAM_ID=${programId}`);
}

function copyIdl(): void {
  const sourceIdl = path.join(CONFIG.anchorRoot, "target/idl/luxhub_marketplace.json");
  if (fs.existsSync(sourceIdl)) {
    fs.copyFileSync(sourceIdl, CONFIG.idlOutput);
    console.log(`✅ Copied IDL to ${CONFIG.idlOutput}`);
  }
}

// ============================================
// DEPLOYMENT ACTIONS
// ============================================

async function buildProgram(): Promise<void> {
  console.log("\n📦 Building program...");
  runCommand("anchor build");
  copyIdl();
}

async function deployProgram(cluster: "devnet" | "mainnet"): Promise<string> {
  console.log(`\n🚀 Deploying to ${cluster}...`);

  // Build first
  await buildProgram();

  // Deploy
  runCommand(`anchor deploy --provider.cluster ${cluster}`);

  // Extract program ID from IDL
  const idl = JSON.parse(fs.readFileSync(
    path.join(CONFIG.anchorRoot, "target/idl/luxhub_marketplace.json"),
    "utf-8"
  ));
  const programId = idl.address || idl.metadata?.address;

  if (!programId) {
    // Fallback: read from keypair
    const keypairPath = path.join(CONFIG.anchorRoot, "target/deploy/luxhub_marketplace-keypair.json");
    const programKeypair = loadKeypair(keypairPath);
    return programKeypair.publicKey.toBase58();
  }

  console.log(`✅ Deployed program: ${programId}`);
  updateEnvFile(programId);

  return programId;
}

async function initializeConfig(
  cluster: "devnet" | "mainnet",
  programId?: string
): Promise<void> {
  console.log(`\n⚙️  Initializing config on ${cluster}...`);

  const provider = getProvider(cluster);
  anchor.setProvider(provider);

  // Load IDL and create program instance
  const idlPath = path.join(CONFIG.anchorRoot, "target/idl/luxhub_marketplace.json");
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf-8"));

  const pid = programId || idl.address || idl.metadata?.address;
  if (!pid) throw new Error("Program ID not found");

  const programPubkey = new PublicKey(pid);

  // Create program with proper typing for Anchor 0.31.0
  const program = new anchor.Program(idl as anchor.Idl, provider);

  // Get config values for this cluster
  const clusterConfig = CONFIG[cluster];
  const squadsMultisig = new PublicKey(clusterConfig.squadsMultisig);
  const squadsAuthority = new PublicKey(clusterConfig.squadsVaultPda);

  // Derive config PDA
  const [configPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("luxhub-config")],
    programPubkey
  );

  console.log(`  Program ID: ${pid}`);
  console.log(`  Config PDA: ${configPda.toBase58()}`);
  console.log(`  Squads Multisig: ${squadsMultisig.toBase58()}`);
  console.log(`  Squads Authority (Vault): ${squadsAuthority.toBase58()}`);

  // Check if config already exists
  try {
    const existingConfig = await provider.connection.getAccountInfo(configPda);
    if (existingConfig) {
      console.log("\n⚠️  Config already initialized!");
      console.log(`  Account exists with ${existingConfig.lamports} lamports`);
      return;
    }
  } catch {
    // Config doesn't exist, proceed with initialization
  }

  // Initialize config
  console.log("\n  Sending initialize_config transaction...");

  const tx = await program.methods
    .initializeConfig(squadsMultisig, squadsAuthority)
    .accounts({
      payer: provider.wallet.publicKey,
      config: configPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  console.log(`✅ Config initialized! Tx: ${tx}`);
  console.log(`   View: https://explorer.solana.com/tx/${tx}?cluster=${cluster}`);
}

// ============================================
// MAIN CLI
// ============================================

type ActionType = "deploy" | "init-config" | "full" | "build";

async function main() {
  const args = process.argv.slice(2);

  // Parse arguments
  let cluster: "devnet" | "mainnet" = "devnet";
  let action: ActionType = "full";
  let programId: string | undefined;

  for (let i = 0; i < args.length; i++) {
    if (args[i] === "--cluster" && args[i + 1]) {
      cluster = args[i + 1] as "devnet" | "mainnet";
    }
    if (args[i] === "--action" && args[i + 1]) {
      action = args[i + 1] as ActionType;
    }
    if (args[i] === "--program-id" && args[i + 1]) {
      programId = args[i + 1];
    }
  }

  console.log("═══════════════════════════════════════════════════");
  console.log("       LuxHub Deployment & Initialization");
  console.log("═══════════════════════════════════════════════════");
  console.log(`  Cluster: ${cluster}`);
  console.log(`  Action:  ${action}`);
  if (programId) console.log(`  Program: ${programId}`);
  console.log("═══════════════════════════════════════════════════");

  try {
    if (action === "build") {
      await buildProgram();
    } else if (action === "deploy") {
      programId = await deployProgram(cluster);
    } else if (action === "init-config") {
      await initializeConfig(cluster, programId);
    } else if (action === "full") {
      programId = await deployProgram(cluster);
      await initializeConfig(cluster, programId);
    }

    console.log("\n═══════════════════════════════════════════════════");
    console.log("                    ✅ COMPLETE");
    console.log("═══════════════════════════════════════════════════\n");

  } catch (error: any) {
    console.error("\n❌ Error:", error.message);
    process.exit(1);
  }
}

main();
