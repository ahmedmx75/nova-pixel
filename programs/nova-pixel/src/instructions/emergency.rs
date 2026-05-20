/// Emergency pause / unpause — requires 2-of-3 multisig.
///
/// Any of the three registered multisig signers can cast a vote.
/// When 2 votes are accumulated the contract immediately pauses.
/// The admin (single key) can unpause after resolving the issue.
///
/// Every action is logged on-chain via events.

use anchor_lang::prelude::*;
use crate::{
    constants::{MULTISIG_COUNT, MULTISIG_THRESHOLD},
    errors::NovaPixelError,
    events::{EmergencyPaused, EmergencyUnpaused},
    state::{GlobalState, TournamentState},
};

// ── Vote to pause (any multisig signer) ──────────────────────────────────────
#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    /// Must be one of the three registered multisig signers.
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global_state"],
        bump  = global_state.bump,
    )]
    pub global_state: Account<'info, GlobalState>,
}

pub fn pause_handler(ctx: Context<EmergencyPause>, reason: String) -> Result<()> {
    let gs = &mut ctx.accounts.global_state;

    // Identify which signer index this is
    let signer_key = ctx.accounts.signer.key();
    let idx = gs
        .multisig_signers
        .iter()
        .position(|k| k == &signer_key)
        .ok_or(NovaPixelError::NotMultisigSigner)?;

    // Idempotent — don't count a duplicate vote
    require!(!gs.pause_votes[idx], NovaPixelError::AlreadyVoted);
    gs.pause_votes[idx] = true;

    let now = Clock::get()?.unix_timestamp;

    emit!(EmergencyPaused {
        reason:    reason.clone(),
        signer:    signer_key,
        timestamp: now,
    });

    // Once threshold is reached, pause immediately
    if gs.pause_vote_count() >= MULTISIG_THRESHOLD {
        // Save previous state so we can restore it on unpause
        // (we always restore to Active; if it was Inactive/Ended that's fine)
        gs.tournament_state = TournamentState::Paused;
    }

    Ok(())
}

// ── Unpause (admin only) ──────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct EmergencyUnpause<'info> {
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global_state"],
        bump  = global_state.bump,
        constraint = admin.key() == global_state.admin @ NovaPixelError::NotAdmin,
    )]
    pub global_state: Account<'info, GlobalState>,
}

pub fn unpause_handler(ctx: Context<EmergencyUnpause>) -> Result<()> {
    let gs = &mut ctx.accounts.global_state;
    require!(gs.is_paused(), NovaPixelError::NotPaused);

    // Resume the tournament in Active state.
    // Admin should only unpause after the security issue is resolved.
    gs.tournament_state = TournamentState::Active;

    // Reset all pause votes so signers can vote again in a future emergency.
    gs.pause_votes = [false; MULTISIG_COUNT];

    emit!(EmergencyUnpaused {
        signer:    ctx.accounts.admin.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
