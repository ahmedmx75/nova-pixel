/// Tournament control — admin-only start / end_round / end_tournament.
///
/// State machine:
///   INACTIVE / ROUND_ENDED / ENDED
///       └──start_tournament()──► ACTIVE
///   ACTIVE ──end_round()──► ROUND_ENDED   (between rounds; no airdrop unlock)
///   ACTIVE / ROUND_ENDED ──end_tournament()──► ENDED
///   ENDED ──finalize_airdrop()──► DISTRIBUTION   (see finalize_airdrop.rs)
///
/// Admin has EXACTLY 3 tournament controls:
///   1. start_tournament  (also starts new rounds)
///   2. end_round         (pauses between rounds)
///   3. end_tournament    (final close before airdrop finalization)
///
/// Admin CANNOT move funds, change tokenomics, or access player balances.

use anchor_lang::prelude::*;
use crate::{
    errors::NovaPixelError,
    events::{RoundEnded as RoundEndedEvent, TournamentEnded, TournamentStarted},
    state::{GlobalState, TournamentState},
};

// ── start_tournament ──────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct StartTournament<'info> {
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global_state"],
        bump  = global_state.bump,
        constraint = admin.key() == global_state.admin @ NovaPixelError::NotAdmin,
    )]
    pub global_state: Account<'info, GlobalState>,
}

pub fn start_handler(ctx: Context<StartTournament>) -> Result<()> {
    let gs = &mut ctx.accounts.global_state;

    require!(!gs.is_paused(), NovaPixelError::ContractPaused);

    // Allow starting from Inactive, RoundEnded, or Ended states.
    require!(
        matches!(
            gs.tournament_state,
            TournamentState::Inactive
                | TournamentState::RoundEnded
                | TournamentState::Ended
        ),
        NovaPixelError::TournamentAlreadyActive
    );

    let now = Clock::get()?.unix_timestamp;

    // Advance round counter when restarting from RoundEnded.
    if gs.tournament_state == TournamentState::RoundEnded {
        gs.current_round = gs
            .current_round
            .checked_add(1)
            .ok_or(NovaPixelError::MathOverflow)?;
    }

    gs.tournament_state      = TournamentState::Active;
    gs.tournament_start_time = now;
    gs.tournament_end_time   = 0;

    emit!(TournamentStarted { timestamp: now });
    Ok(())
}

// ── end_round ─────────────────────────────────────────────────────────────────
/// Ends the current round WITHOUT triggering airdrop distribution.
/// Players may call sell_ingame (with penalty) while in RoundEnded state.
/// Admin can then call start_tournament() again to begin the next round,
/// or call end_tournament() to close the tournament for good.
#[derive(Accounts)]
pub struct EndRound<'info> {
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global_state"],
        bump  = global_state.bump,
        constraint = admin.key() == global_state.admin @ NovaPixelError::NotAdmin,
    )]
    pub global_state: Account<'info, GlobalState>,
}

pub fn end_round_handler(ctx: Context<EndRound>) -> Result<()> {
    let gs = &mut ctx.accounts.global_state;

    require!(!gs.is_paused(), NovaPixelError::ContractPaused);
    require!(gs.is_active(), NovaPixelError::TournamentNotActive);

    let now = Clock::get()?.unix_timestamp;
    gs.tournament_state    = TournamentState::RoundEnded;
    gs.tournament_end_time = now;

    emit!(RoundEndedEvent {
        round:                gs.current_round,
        timestamp:            now,
        total_correct_pixels: gs.total_correct_pixels,
    });

    Ok(())
}

// ── end_tournament ────────────────────────────────────────────────────────────
/// Final close of the tournament.  Does NOT unlock the airdrop wallet —
/// admin must call finalize_airdrop() separately to trigger distribution.
/// Callable from Active or RoundEnded states.
#[derive(Accounts)]
pub struct EndTournament<'info> {
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global_state"],
        bump  = global_state.bump,
        constraint = admin.key() == global_state.admin @ NovaPixelError::NotAdmin,
    )]
    pub global_state: Account<'info, GlobalState>,
}

pub fn end_handler(ctx: Context<EndTournament>) -> Result<()> {
    let gs = &mut ctx.accounts.global_state;

    require!(!gs.is_paused(), NovaPixelError::ContractPaused);
    require!(
        gs.is_active() || gs.is_round_ended(),
        NovaPixelError::TournamentNotActive
    );

    let now = Clock::get()?.unix_timestamp;
    gs.tournament_state    = TournamentState::Ended;
    gs.tournament_end_time = now;
    // NOTE: airdrop_wallet is NOT unlocked here.
    // Admin must call finalize_airdrop() to unlock and allow claims.

    emit!(TournamentEnded {
        timestamp:            now,
        total_correct_pixels: gs.total_correct_pixels,
        total_nvpx_in_game:   gs.total_nvpx_in_game,
    });

    Ok(())
}
