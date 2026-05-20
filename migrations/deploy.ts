/**
 * Nova Pixel — deploy & initialize script
 *
 * Run with:  anchor migrate --provider.cluster devnet
 *
 * The on-chain program uses a single `initialize` instruction that creates
 * all 7 wallet PDAs AND the 6 NVPX token ATAs in one transaction.
 *
 * Environment variables:
 *   GAME_SERVER     — game server pubkey (base58); defaults to admin
 *   MULTISIG_1/2/3  — multisig signer pubkeys; default to admin
 *   USE_DEVNET      — "false" for mainnet time-locks; anything else uses short devnet locks
 */

import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import {
  createMint,
  mintTo,
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import * as fs from "fs";

const NVPX_DECIMALS = 9;

module.exports = async function (provider: anchor.AnchorProvider) {
  anchor.setProvider(provider);

  const program = anchor.workspace.NovaPixel;
  const admin   = provider.wallet as anchor.Wallet;

  console.log("Nova Pixel Deploy Script");
  console.log("========================");
  console.log("Admin:   ", admin.publicKey.toBase58());
  console.log("Program: ", program.programId.toBase58());
  console.log("Network: ", provider.connection.rpcEndpoint);

  // ── Configuration ─────────────────────────────────────────────────────────
  const useDevnet = process.env.USE_DEVNET !== "false";
  const gameServerPubkey = process.env.GAME_SERVER
    ? new PublicKey(process.env.GAME_SERVER)
    : admin.publicKey;

  const multisigSigners: [PublicKey, PublicKey, PublicKey] = [
    process.env.MULTISIG_1 ? new PublicKey(process.env.MULTISIG_1) : admin.publicKey,
    process.env.MULTISIG_2 ? new PublicKey(process.env.MULTISIG_2) : admin.publicKey,
    process.env.MULTISIG_3 ? new PublicKey(process.env.MULTISIG_3) : admin.publicKey,
  ];

  // ── Create NVPX mint ────────────────────────────────────────────────────
  console.log("\n1. Creating NVPX mint...");
  const nvpxMint = await createMint(
    provider.connection,
    (admin as any).payer,
    admin.publicKey,
    null,
    NVPX_DECIMALS
  );
  console.log("   NVPX Mint:", nvpxMint.toBase58());

  // ── Derive PDAs ──────────────────────────────────────────────────────────
  const pda = (seeds: Buffer[]) =>
    anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

  const ata = (authority: PublicKey) =>
    anchor.utils.token.associatedAddress({ mint: nvpxMint, owner: authority });

  const [globalState]          = pda([Buffer.from("global_state")]);
  const [liquidityWalletState] = pda([Buffer.from("liquidity_wallet")]);
  const [airdropWalletState]   = pda([Buffer.from("airdrop_wallet")]);
  const [teamWalletState]      = pda([Buffer.from("team_wallet")]);
  const [devWalletState]       = pda([Buffer.from("development_wallet")]);
  const [burnWalletState]      = pda([Buffer.from("burn_wallet")]);
  const [reserveWalletState]   = pda([Buffer.from("reserve_wallet")]);
  const [buybackWalletState]   = pda([Buffer.from("buyback_wallet")]);

  // ── Initialize: create all PDAs + ATAs in one call ───────────────────────
  console.log("\n2. Initializing program (PDAs + token accounts)...");
  try {
    await program.methods
      .initialize({
        gameServer:          gameServerPubkey,
        multisigSigners,
        useDevnetTimelocks:  useDevnet,
      })
      .accounts({
        admin:                   admin.publicKey,
        globalState,
        nvpxMint,
        liquidityWalletState,
        airdropWalletState,
        teamWalletState,
        developmentWalletState:  devWalletState,
        burnWalletState,
        reserveWalletState,
        buybackWalletState,
        liquidityTokenAccount:   ata(liquidityWalletState),
        airdropTokenAccount:     ata(airdropWalletState),
        teamTokenAccount:        ata(teamWalletState),
        developmentTokenAccount: ata(devWalletState),
        burnTokenAccount:        ata(burnWalletState),
        reserveTokenAccount:     ata(reserveWalletState),
        tokenProgram:            TOKEN_PROGRAM_ID,
        associatedTokenProgram:  ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram:           anchor.web3.SystemProgram.programId,
        rent:                    anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();
    console.log("   GlobalState:  ", globalState.toBase58());
  } catch (e: any) {
    if (e.message?.includes("already in use") || e.message?.includes("already been processed")) {
      console.log("   (already initialized, continuing...)");
    } else {
      throw e;
    }
  }

  // ── Mint NVPX to wallet ATAs ─────────────────────────────────────────────
  console.log("\n3. Minting NVPX to wallet ATAs...");
  const M = (millions: number) => BigInt(millions * 1_000_000) * BigInt(10 ** NVPX_DECIMALS);

  const allocations: Array<[PublicKey, bigint, string]> = [
    [liquidityWalletState, M(200), "Liquidity (200M)"],
    [airdropWalletState,   M(400), "Airdrop   (400M)"],
    [teamWalletState,      M(100), "Team      (100M)"],
    [devWalletState,       M(100), "Development(100M)"],
    [burnWalletState,      M(150), "Burn      (150M)"],
    [reserveWalletState,   M(50),  "Reserve    (50M)"],
  ];
  // Total: 1,000,000,000 NVPX

  for (const [walletPda, amount, label] of allocations) {
    const dest = ata(walletPda);
    await mintTo(
      provider.connection,
      (admin as any).payer,
      nvpxMint,
      dest,
      admin.publicKey,
      amount
    );
    console.log(`   ${label} → ${dest.toBase58()}`);
  }

  // ── Save deployment info ─────────────────────────────────────────────────
  const deployment = {
    programId:          program.programId.toBase58(),
    nvpxMint:           nvpxMint.toBase58(),
    globalState:        globalState.toBase58(),
    liquidityWallet:    ata(liquidityWalletState).toBase58(),
    airdropWallet:      ata(airdropWalletState).toBase58(),
    teamWallet:         ata(teamWalletState).toBase58(),
    developmentWallet:  ata(devWalletState).toBase58(),
    burnWallet:         ata(burnWalletState).toBase58(),
    reserveWallet:      ata(reserveWalletState).toBase58(),
    buybackWallet:      buybackWalletState.toBase58(),
    admin:              admin.publicKey.toBase58(),
    gameServer:         gameServerPubkey.toBase58(),
    multisigSigners:    multisigSigners.map(k => k.toBase58()),
    deployedAt:         new Date().toISOString(),
    network:            provider.connection.rpcEndpoint.includes("devnet") ? "devnet" : "localnet",
    totalSupply:        "1,000,000,000 NVPX",
    decimals:           NVPX_DECIMALS,
  };

  const outPath = "deployment.json";
  fs.writeFileSync(outPath, JSON.stringify(deployment, null, 2));
  console.log("\n✅ Deployment complete — saved to", outPath);
  console.log(JSON.stringify(deployment, null, 2));
};
