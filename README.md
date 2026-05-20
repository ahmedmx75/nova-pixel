# Nova Pixel — Solana Smart Contract

**Token:** NVPX • **Chain:** Solana • **Framework:** Anchor • **DEX:** Jupiter v6

---

## Project Structure

```
nova-pixel/
├── Anchor.toml                      # Anchor configuration
├── Cargo.toml                       # Workspace root
├── package.json / tsconfig.json     # TypeScript tooling
├── migrations/deploy.ts             # One-time deploy + init script
├── tests/nova-pixel.ts              # Full test suite (12 scenarios)
└── programs/nova-pixel/src/
    ├── lib.rs                       # Program entry point (all instructions)
    ├── constants.rs                 # Token amounts, time-locks, prices
    ├── errors.rs                    # All error codes
    ├── events.rs                    # On-chain events
    ├── jupiter.rs                   # Jupiter v6 CPI helper
    ├── state/
    │   ├── global_state.rs          # GlobalState PDA + TournamentState enum
    │   ├── player_account.rs        # PlayerAccount PDA + Shield struct
    │   └── wallet_state.rs          # WalletState PDA
    └── instructions/
        ├── initialize.rs            # Deploy all 7 wallet PDAs
        ├── connect_player.rs        # Register wallet + team
        ├── buy_package.rs           # SOL → Jupiter → NVPX → airdrop_wallet
        ├── color_pixel.rs           # Game server submits 2× pixel reward
        ├── capture_pixel.rs         # Game server submits pixel capture
        ├── buy_item.rs              # Shield / Rocket (SOL → buyback_wallet)
        ├── sell_ingame.rs           # In-game sell with 50% penalty + 2% tax
        ├── tournament.rs            # start_tournament / end_tournament
        ├── emergency.rs             # 2-of-3 multisig pause / admin unpause
        ├── claim_airdrop.rs         # Proportional airdrop after tournament
        └── admin.rs                 # burn_tokens, admin_buyback, rocket_resolve
```

---

## Program PDAs

| PDA Seed                | Allocation       | Lock             | Note                          |
|-------------------------|------------------|------------------|-------------------------------|
| `global_state`          | —                | —                | Tournament state machine      |
| `sol_vault`             | —                | —                | Temporary SOL for Jupiter CPI |
| `liquidity_wallet`      | 200M NVPX        | 24 months        | Streamflow-compatible         |
| `airdrop_wallet`        | 400M NVPX        | Until tournament ends | Player rewards + airdrop |
| `team_wallet`           | 100M NVPX        | 180 days         |                               |
| `development_wallet`    | 100M NVPX        | 90 days          | Admin-controlled after lock   |
| `burn_wallet`           | 150M NVPX        | One-time burn    | Phase 04 burn                 |
| `reserve_wallet`        | 50M NVPX         | None             | Admin-controlled              |
| `buyback_wallet`        | Receives SOL     | None             | SOL from item purchases       |
| `player` (per-user)     | Per-player state | —                | Attempts, balance, shields    |

---

## Tournament State Machine

```
INACTIVE ──start_tournament()──► ACTIVE ──end_tournament()──► ENDED ──► DISTRIBUTION
                                     │
                              emergency_pause()
                              (2-of-3 multisig)
                                     │
                                  PAUSED
                                     │
                            emergency_unpause()
                              (admin only)
                                     │
                                  ACTIVE
```

**Active state:** purchases open, withdrawals blocked, selling allowed (with penalty)  
**Ended state:** purchases blocked, airdrop claims open, canvas frozen  
**Paused state:** everything blocked, emergency only

---

## Core Mechanics

### Package Purchase (buy_package)
```
Player SOL → sol_vault PDA → Jupiter CPI → NVPX → airdrop_wallet ATA
                                                         ↓
                                              player.attempts_balance += N
```

### 2× Pixel Reward (color_pixel — game server)
```
pixel_value = 1,000 NVPX
reward      = 2,000 NVPX  →  player.in_game_nvpx_balance
                              drawn from airdrop_wallet pool
```

### Pixel Capture (capture_pixel — game server)
```
defender loses 2× reward from in_game_nvpx_balance
attacker gains  same amount
(shield check runs first — blocked if defender has active shield)
```

### Sell Penalty (sell_ingame — active tournament only)
```
sell 2,000 NVPX:
  penalty  = 1,000 NVPX  →  airdrop pool
  tax      =    20 NVPX  →  development_wallet
  received =   980 NVPX  →  player keeps
```

### Airdrop Distribution (claim_airdrop — after tournament)
```
player_share = (player_correct_pixels / total_correct_pixels) × 400,000,000 NVPX
```

---

## Admin Controls (exactly 3 tournament functions)

| Function               | Who       | Effect                                |
|------------------------|-----------|---------------------------------------|
| `start_tournament()`   | Admin     | Opens all purchases                   |
| `end_tournament()`     | Admin     | Closes purchases, opens withdrawals   |
| `emergency_pause()`    | 2-of-3 MS | Halts everything immediately          |
| `emergency_unpause()`  | Admin     | Resumes after security fix            |

Admin **cannot**: move funds manually, change tokenomics, access player balances, modify airdrop allocations.

---

## Jupiter Integration

The contract uses a pass-through CPI pattern:

1. **Off-chain:** call `GET /quote` then `POST /swap-instructions` from Jupiter API v6
2. **Pass to contract:** `jupiter_data` (raw instruction bytes) + `remaining_accounts` (all Jupiter route accounts)
3. **On-chain:** `sol_vault` PDA signs via `invoke_signed` → tokens arrive at `airdrop_wallet` ATA

```
Jupiter API → route bytes → buy_package(jupiter_data, remaining_accounts)
                                   ↓
                    invoke_signed(JUPITER_PROGRAM_ID, sol_vault PDA)
                                   ↓
                    NVPX deposited to airdrop_wallet ATA
```

---

## Devnet Setup & Testing

### Prerequisites
```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm install 0.30.1 && avm use 0.30.1

# Install Node deps
yarn install
```

### Build & Deploy (Devnet)
```bash
# 1. Generate program keypair
anchor keys generate

# 2. Update program ID in Anchor.toml and lib.rs
anchor keys list  # copy the key

# 3. Build
anchor build

# 4. Deploy to devnet
anchor deploy --provider.cluster devnet

# 5. Initialize (deploys all PDAs and mints NVPX)
USE_DEVNET=true \
GAME_SERVER=<game-server-pubkey> \
MULTISIG_1=<ms1> MULTISIG_2=<ms2> MULTISIG_3=<ms3> \
anchor migrate --provider.cluster devnet
```

### Run Tests
```bash
anchor test --provider.cluster devnet
```

### Test Checklist (per spec)
- [x] `buy_package` — NVPX goes to `airdrop_wallet`
- [x] 2× pixel reward — player receives double in-game balance
- [x] Pixel capture — NVPX transfers defender → attacker
- [x] `buy_item` — SOL goes to `buyback_wallet`, NOT airdrop
- [x] `sell_ingame` penalty — 50% loss + 2% tax (2,000 → 980)
- [x] `start_tournament` / `end_tournament`
- [x] `emergency_pause` / `unpause` (2-of-3 multisig)
- [x] `claim_airdrop` proportional formula
- [x] Time-lock enforcement
- [x] Reentrancy attack prevention (Anchor's exclusive borrow model)
- [x] All events emitted correctly

---

## Security Features

| Threat                    | Protection                                                |
|---------------------------|-----------------------------------------------------------|
| Reentrancy                | Anchor exclusive mutable borrows; checks before transfers |
| Integer overflow/underflow | All math via `checked_add/sub/mul/div`                   |
| Unauthorized fund access  | PDA seed constraints on every sensitive account           |
| Admin overreach           | Admin restricted to 3 tournament functions                |
| Emergency abuse           | Pause requires 2-of-3 multisig                           |
| Time-lock bypass          | `lock_until` enforced by on-chain `Clock` timestamp       |
| Jupiter spoofing          | Program ID validated: `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4` |
| Double airdrop claim      | `airdrop_claimed` bool in player PDA                     |

---

## Roadmap Reference

| Phase | Timeline  | Status      |
|-------|-----------|-------------|
| 00    | Month 1   | IN PROGRESS — Community Launch |
| 01    | Week 4    | NVPX goes live on-chain         |
| 02    | Month 2   | Airdrop registration opens      |
| 03    | Month 3   | CEX listing agreements          |
| 04    | Month 4+  | `burn_tokens()` — burns 150M NVPX |

---

*Nova Pixel · NVPX · Solana Mainnet · Built with Anchor*
