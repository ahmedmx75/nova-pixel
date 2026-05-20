//! Nova Pixel — Blockchain Pixel War Game
//!
//! Token:   NVPX (SPL, 1,000,000,000 total supply, 9 decimals)
//! Chain:   Solana (Devnet → Mainnet)
//! Framework: Anchor
//!
//! # Program PDAs
//!   global_state          seeds: [b"global_state"]
//!   sol_vault             seeds: [b"sol_vault"]
//!   liquidity_wallet      seeds: [b"liquidity_wallet"]
//!   airdrop_wallet        seeds: [b"airdrop_wallet"]
//!   team_wallet           seeds: [b"team_wallet"]
//!   development_wallet    seeds: [b"development_wallet"]
//!   burn_wallet           seeds: [b"burn_wallet"]
//!   reserve_wallet        seeds: [b"reserve_wallet"]
//!   buyback_wallet        seeds: [b"buyback_wallet"]
//!   player_account        seeds: [b"player", wallet.key]
//!
//! # Admin controls (exactly 3 tournament functions)
//!   start_tournament / end_tournament / emergency_pause + unpause

use anchor_lang::prelude::*;

// ── Declare program ID ────────────────────────────────────────────────────────
// Replace with the result of `anchor keys list` before deployment.
declare_id!("DRD8K7Ywmpy4JqNE473uBTs6jaf5ajrQ32FxoxzbRoGf");

pub mod constants;
pub mod errors;
pub mod events;
pub mod jupiter;
pub mod state;
pub mod instructions;

use instructions::*;

#[program]
pub mod nova_pixel {
    use super::*;

    // ── Initialisation ────────────────────────────────────────────────────────

    /// Deploy all PDAs and wallet states.  Called once by admin.
    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        initialize::handler(ctx, params)
    }

    // ── Player ────────────────────────────────────────────────────────────────

    /// Register a wallet and join a team (0 / 1 / 2).
    pub fn connect_player(ctx: Context<ConnectPlayer>, team: u8) -> Result<()> {
        connect_player::handler(ctx, team)
    }

    // ── Purchases ─────────────────────────────────────────────────────────────

    /// Buy Starter / Advanced / Pro package — routes SOL through Jupiter → NVPX.
    /// Pass Jupiter swap data and all required accounts as remaining_accounts.
    pub fn buy_package(
        ctx:          Context<BuyPackage>,
        package_type: u8,
        jupiter_data: Vec<u8>,
        sol_amount:   u64,
    ) -> Result<()> {
        buy_package::handler(ctx, package_type, jupiter_data, sol_amount)
    }

    /// Buy a Shield 3×3, Shield 5×5, or Rocket.  SOL goes to buyback_wallet.
    pub fn buy_item(
        ctx:       Context<BuyItem>,
        item_type: u8,
        target_x:  u16,
        target_y:  u16,
        defender:  Option<Pubkey>,
    ) -> Result<()> {
        buy_item::handler(ctx, item_type, target_x, target_y, defender)
    }

    // ── Game events (game server / oracle) ────────────────────────────────────

    /// Record a pixel coloring event and pay the 2× reward from the airdrop pool.
    pub fn color_pixel(
        ctx:           Context<ColorPixel>,
        player_wallet: Pubkey,
        x:             u16,
        y:             u16,
        pixel_value:   u64,
        is_correct:    bool,
    ) -> Result<()> {
        color_pixel::handler(ctx, player_wallet, x, y, pixel_value, is_correct)
    }

    /// Record a pixel capture: attacker takes the 2× reward from defender.
    pub fn capture_pixel(
        ctx:             Context<CapturePixel>,
        attacker_wallet: Pubkey,
        defender_wallet: Pubkey,
        x:               u16,
        y:               u16,
        nvpx_reward:     u64,
    ) -> Result<()> {
        capture_pixel::handler(ctx, attacker_wallet, defender_wallet, x, y, nvpx_reward)
    }

    /// Game server removes a defender's shield after a Rocket purchase resolves.
    pub fn rocket_resolve(
        ctx:              Context<RocketResolve>,
        defender_wallet:  Pubkey,
        target_x:         u16,
        target_y:         u16,
    ) -> Result<()> {
        admin::rocket_resolve_handler(ctx, defender_wallet, target_x, target_y)
    }

    // ── In-game economy ───────────────────────────────────────────────────────

    /// Sell in-game NVPX during an active tournament.  50% penalty + 2% tax applied.
    pub fn sell_ingame(ctx: Context<SellIngame>, amount: u64) -> Result<()> {
        sell_ingame::handler(ctx, amount)
    }

    /// Claim proportional airdrop share after the tournament ends.
    pub fn claim_airdrop(ctx: Context<ClaimAirdrop>) -> Result<()> {
        claim_airdrop::handler(ctx)
    }

    // ── Tournament controls (admin only — exactly 3) ──────────────────────────

    /// ADMIN: open the tournament.
    pub fn start_tournament(ctx: Context<StartTournament>) -> Result<()> {
        tournament::start_handler(ctx)
    }

    /// ADMIN: close the tournament and begin airdrop distribution phase.
    pub fn end_tournament(ctx: Context<EndTournament>) -> Result<()> {
        tournament::end_handler(ctx)
    }

    /// MULTISIG: cast a pause vote (2-of-3 required to pause).
    pub fn emergency_pause(ctx: Context<EmergencyPause>, reason: String) -> Result<()> {
        emergency::pause_handler(ctx, reason)
    }

    /// ADMIN: unpause after security issue is resolved.
    pub fn emergency_unpause(ctx: Context<EmergencyUnpause>) -> Result<()> {
        emergency::unpause_handler(ctx)
    }

    // ── Admin — wallet management ─────────────────────────────────────────────

    /// ADMIN: one-time burn of the 150,000,000 NVPX burn allocation (Phase 04).
    pub fn burn_tokens(ctx: Context<BurnTokens>) -> Result<()> {
        admin::burn_handler(ctx)
    }

    /// ADMIN: use buyback SOL to buy NVPX from Jupiter DEX.
    pub fn admin_buyback(
        ctx:          Context<AdminBuyback>,
        sol_amount:   u64,
        jupiter_data: Vec<u8>,
    ) -> Result<()> {
        admin::buyback_handler(ctx, sol_amount, jupiter_data)
    }

    /// ADMIN: withdraw from a time-unlocked development or reserve wallet.
    pub fn withdraw_locked(ctx: Context<WithdrawLocked>, amount: u64) -> Result<()> {
        admin::withdraw_locked_handler(ctx, amount)
    }

    /// ADMIN: permanently revoke mint + freeze authority — token becomes fully immutable.
    /// Call once after full supply is minted. IRREVERSIBLE.
    pub fn revoke_authorities(ctx: Context<RevokeAuthorities>) -> Result<()> {
        revoke_authorities::handler(ctx)
    }
}
