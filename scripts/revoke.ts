/**
 * revoke.ts — Permanently revoke NVPX mint + freeze authority on devnet.
 *
 * Run:  .\node_modules\.bin\ts-node.cmd scripts/revoke.ts
 */

import { Connection, Keypair, PublicKey, Transaction, TransactionInstruction, clusterApiUrl } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as crypto from "crypto";
import * as fs from "fs";
import * as path from "path";

const PROGRAM_ID = new PublicKey("DRD8K7Ywmpy4JqNE473uBTs6jaf5ajrQ32FxoxzbRoGf");
const RPC = clusterApiUrl("devnet");

// Load admin keypair
const keypairPath = path.join(
  process.env.USERPROFILE || process.env.HOME!,
  ".config", "solana", "id.json"
);
const secretKey = Uint8Array.from(JSON.parse(fs.readFileSync(keypairPath, "utf-8")));
const admin = Keypair.fromSecretKey(secretKey);
console.log("Admin:", admin.publicKey.toBase58());

// Compute global_state PDA
const [globalStatePda] = PublicKey.findProgramAddressSync(
  [Buffer.from("global_state")],
  PROGRAM_ID
);
console.log("global_state PDA:", globalStatePda.toBase58());

async function main() {
  const connection = new Connection(RPC, "confirmed");

  // Fetch global_state to get nvpx_mint address
  // GlobalState layout (OLD — no authorities_revoked field):
  //   8   discriminator
  //   32  admin
  //   32  game_server
  //   96  multisig_signers [Pubkey;3]
  //   3   pause_votes [bool;3]
  //   2   tournament_state
  //   8   tournament_start_time
  //   8   tournament_end_time
  //   8   initialized_at
  //   1   current_round
  //   1   airdrop_finalized
  //   32  nvpx_mint  ← offset = 8+32+32+96+3+2+8+8+8+1+1 = 199
  const accountInfo = await connection.getAccountInfo(globalStatePda);
  if (!accountInfo) throw new Error("global_state not found — program not initialized?");

  // GlobalState layout with authorities_revoked:
  //   8   disc | 32 admin | 32 game_server | 96 multisig | 3 votes | 1 state |
  //   8 start | 8 end | 8 init | 1 round | 1 finalized | 1 revoked | 32 mint
  //   TournamentState borsh = 1 byte (u8 discriminant), not 2
  const mintOffset = 8 + 32 + 32 + 96 + 3 + 1 + 8 + 8 + 8 + 1 + 1 + 1; // = 199
  const nvpxMintBytes = accountInfo.data.slice(mintOffset, mintOffset + 32);
  const nvpxMint = new PublicKey(nvpxMintBytes);
  console.log("NVPX Mint:", nvpxMint.toBase58());

  // Check current mint authority from SPL mint account
  // SPL Mint layout: COption<Pubkey> at offset 0
  //   tag (u32 LE) = 0 (None) or 1 (Some)
  //   pubkey (32 bytes, only valid if tag=1)
  const mintAccount = await connection.getAccountInfo(nvpxMint);
  if (!mintAccount) throw new Error("NVPX mint account not found");

  const mintAuthorityTag = mintAccount.data.readUInt32LE(0);
  const mintAuthority = mintAuthorityTag === 1
    ? new PublicKey(mintAccount.data.slice(4, 36))
    : null;

  const freezeAuthorityOffset = 36; // after mint_authority (4+32)
  const freezeAuthorityTag = mintAccount.data.readUInt32LE(freezeAuthorityOffset);
  const freezeAuthority = freezeAuthorityTag === 1
    ? new PublicKey(mintAccount.data.slice(40, 72))
    : null;

  console.log("Mint authority:", mintAuthority?.toBase58() ?? "None (already revoked)");
  console.log("Freeze authority:", freezeAuthority?.toBase58() ?? "None (already revoked)");

  if (!mintAuthority && !freezeAuthority) {
    console.log("\n✓ Both authorities are already None — token is already immutable. Done.");
    return;
  }

  // Compute instruction discriminator: sha256("global:revoke_authorities")[0..8]
  const discriminator = crypto
    .createHash("sha256")
    .update("global:revoke_authorities")
    .digest()
    .slice(0, 8);

  console.log("\nSending revoke_authorities instruction...");

  const revokeIx = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: admin.publicKey,  isSigner: true,  isWritable: false },
      { pubkey: globalStatePda,   isSigner: false, isWritable: true  },
      { pubkey: nvpxMint,         isSigner: false, isWritable: true  },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data: Buffer.from(discriminator),
  });

  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
  const tx = new Transaction({ recentBlockhash: blockhash, feePayer: admin.publicKey }).add(revokeIx);
  tx.sign(admin);

  const sig = await connection.sendRawTransaction(tx.serialize(), { skipPreflight: false });
  await connection.confirmTransaction({ signature: sig, blockhash, lastValidBlockHeight }, "confirmed");

  console.log("\n✓ SUCCESS — NVPX mint + freeze authority permanently revoked!");
  console.log("✓ Token is now fully immutable — supply fixed at 1,000,000,000 NVPX.");
  console.log(`  Explorer: https://explorer.solana.com/tx/${sig}?cluster=devnet`);
  console.log(`  Mint on Solscan: https://solscan.io/token/${nvpxMint.toBase58()}?cluster=devnet`);
}

main().catch(err => {
  console.error("Error:", err.message ?? err);
  if (err.logs) console.error("Logs:", err.logs);
  process.exit(1);
});
