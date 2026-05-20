/// finalize_airdrop — ADMIN only.
///
/// Called by admin after end_tournament() to open the 40% community airdrop
/// pool for player claims.  This is a two-step design:
///
///   1. Admin calls end_tournament()  — closes active play, no distribution.
///   2. Admin calls finalize_airdrop() — unlocks airdrop wallet, transitions
///      state to Distribution, sets the final pixel denominator.
///
/// After finalize_airdrop():
///   - Players call claim_airdrop() to receive their proportional share.
///   - Formula: share = (player_correct_pixels / total_correct_pixels) × 400M NVPX
///   - Pixel counts accumulate across ALL rounds in the tournament.
///
/// # Security
///   - Only callable once (airdrop_finalized guard).
///   - Requires tournament to be in Ended state.
///   - Admin cannot alter player pixel counts or the total pool size.

use anchor_lang::prelude::*;
use crate::{
    errors::NovaPixelError,
    events::AirdropFinalized,
    state::{GlobalState, TournamentState, WalletState},
};

#[derive(Accounts)]
pub struct FinalizeAirdrop<'info> {
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global_state"],
        bump  = global_state.bump,
        constraint = admin.key() == global_state.admin @ NovaPixelError::NotAdmin,
    )]
    pub global_state: Account<'info, GlobalState>,

    /// The airdrop wallet — unlocked here so claim_airdrop() can pull from it.
    #[account(
        mut,
        seeds  = [b"airdrop_wallet"],
        bump   = airdrop_wallet_state.bump,
    )]
    pub airdrop_wallet_state: Account<'info, WalletState>,
}

pub fn handler(ctx: Context<FinalizeAirdrop>) -> Result<()> {
    let gs = &mut ctx.accounts.global_state;

    require!(!gs.is_paused(), NovaPixelError::ContractPaused);

    // Must be in Ended state (end_tournament() must have been called first).
    require!(
        gs.tournament_state == TournamentState::Ended,
        NovaPixelError::TournamentNotFinalizable
    );

    // Can only finalize once.
    require!(!gs.airdrop_finalized, NovaPixelError::AirdropAlreadyFinalized);

    // At least one pixel must have been colored for proportional math to work.
    require!(gs.total_correct_pixels > 0, NovaPixelError::DivisionByZero);

    let now = Clock::get()?.unix_timestamp;

    // ── Unlock airdrop wallet ─────────────────────────────────────────────────
    // Setting lock_until = 0 allows claim_airdrop() to transfer from the pool.
    ctx.accounts.airdrop_wallet_state.lock_until = 0;

    // ── Update global state ───────────────────────────────────────────────────
    gs.airdrop_finalized   = true;
    gs.tournament_state    = TournamentState::Distribution;

    emit!(AirdropFinalized {
        timestamp:            now,
        total_correct_pixels: gs.total_correct_pixels,
        total_rounds:         gs.current_round.saturating_add(1),
    });

    Ok(())
}
