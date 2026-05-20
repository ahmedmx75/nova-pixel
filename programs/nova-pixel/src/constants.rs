// Token decimals
pub const NVPX_DECIMALS: u64 = 1_000_000_000; // 10^9

// ── Wallet allocations (NVPX base units with 9 decimals) ──────────────────────
pub const LIQUIDITY_ALLOCATION:   u64 = 200_000_000 * NVPX_DECIMALS;
pub const AIRDROP_ALLOCATION:     u64 = 400_000_000 * NVPX_DECIMALS;
pub const TEAM_ALLOCATION:        u64 = 100_000_000 * NVPX_DECIMALS;
pub const DEVELOPMENT_ALLOCATION: u64 = 100_000_000 * NVPX_DECIMALS;
pub const BURN_ALLOCATION:        u64 = 150_000_000 * NVPX_DECIMALS;
pub const RESERVE_ALLOCATION:     u64 =  50_000_000 * NVPX_DECIMALS;
pub const TOTAL_SUPPLY:           u64 = 1_000_000_000 * NVPX_DECIMALS;

// ── Time locks (seconds) ──────────────────────────────────────────────────────
pub const LIQUIDITY_LOCK_DURATION:   i64 = 60 * 60 * 24 * 730; // 24 months
pub const TEAM_LOCK_DURATION:        i64 = 60 * 60 * 24 * 180; // 180 days
pub const DEVELOPMENT_LOCK_DURATION: i64 = 60 * 60 * 24 *  90; //  90 days
pub const DEVNET_LOCK_DURATION:      i64 = 3_600;               //  1 hour (testing)

// ── Package definitions ───────────────────────────────────────────────────────
pub const PACKAGE_STARTER:  u8 = 0;
pub const PACKAGE_ADVANCED: u8 = 1;
pub const PACKAGE_PRO:      u8 = 2;

pub const STARTER_ATTEMPTS:  u32 = 10;
pub const ADVANCED_ATTEMPTS: u32 = 50;
pub const PRO_ATTEMPTS:      u32 = 200;

// NVPX equivalent the package represents (used to size the Jupiter swap)
pub const STARTER_NVPX_EQUIV:  u64 =  10_000 * NVPX_DECIMALS;
pub const ADVANCED_NVPX_EQUIV: u64 =  50_000 * NVPX_DECIMALS;
pub const PRO_NVPX_EQUIV:      u64 = 200_000 * NVPX_DECIMALS;

// ── Item definitions ──────────────────────────────────────────────────────────
pub const ITEM_SHIELD_3X3: u8 = 0;
pub const ITEM_SHIELD_5X5: u8 = 1;
pub const ITEM_ROCKET:     u8 = 2;

// Item prices in lamports (0.05 / 0.10 / 0.20 SOL)
pub const SHIELD_3X3_PRICE_LAMPORTS: u64 =  50_000_000;
pub const SHIELD_5X5_PRICE_LAMPORTS: u64 = 100_000_000;
pub const ROCKET_PRICE_LAMPORTS:     u64 = 200_000_000;

// Shield coverage radius (size = diameter)
pub const SHIELD_3X3_SIZE: u8 = 3;
pub const SHIELD_5X5_SIZE: u8 = 5;

// Shield active duration (24 h)
pub const SHIELD_DURATION_SECS: i64 = 86_400;

// ── Economy mechanics ─────────────────────────────────────────────────────────
pub const PIXEL_REWARD_MULTIPLIER: u64 = 2;  // 2× NVPX value per correct pixel

// Sell penalty during active tournament (basis points, 1 bps = 0.01 %)
pub const SELL_PENALTY_BPS: u64 = 5_000;     // 50 %
pub const SELL_TAX_BPS:     u64 =   200;     //  2 %
pub const BPS_DENOMINATOR:  u64 = 10_000;

// ── Multisig ──────────────────────────────────────────────────────────────────
pub const MULTISIG_THRESHOLD: usize = 2;
pub const MULTISIG_COUNT:     usize = 3;

// ── Capacity limits ───────────────────────────────────────────────────────────
pub const MAX_SHIELDS:    usize = 10;
pub const MAX_TEAMS:      u8    =  3;
pub const CANVAS_MAX_X:   u16   = 1000;
pub const CANVAS_MAX_Y:   u16   = 1000;

// ── Program IDs ───────────────────────────────────────────────────────────────
pub const JUPITER_PROGRAM_ID_STR: &str =
    "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
