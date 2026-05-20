/// claim_airdrop — player calls this after the tournament ends to receive
/// their proportional share of the 400,000,000 NVPX airdrop pool.
///
/// Formula:
///   player_share = (player_correct_pixels / total_correct_pixels)
///                  × AIRDROP_ALLOCATION
///
/// The actual token transfer goes from the airdrop_wallet ATA → player ATA.
/// Players who sold in-game during the tournament have a reduced allocation
/// (their airdrop_allocation was decremented in sell_ingame).

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    constants::AIRDROP_ALLOCATION,
    errors::NovaPixelError,
    events::AirdropClaimed,
    state::{GlobalState, PlayerAccount, WalletState},
};

#[derive(Accounts)]
pub struct ClaimAirdrop<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds  = [b"player", player.key().as_ref()],
        bump   = player_account.bump,
        constraint = player_account.is_initialized @ NovaPixelError::PlayerNotInitialized,
    )]
    pub player_account: Account<'info, PlayerAccount>,

    #[account(seeds = [b"global_state"], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds  = [b"airdrop_wallet"],
        bump   = airdrop_wallet_state.bump,
    )]
    pub airdrop_wallet_state: Account<'info, WalletState>,

    /// Airdrop pool token account (source of airdrop tokens).
    #[account(
        mut,
        constraint = airdrop_token_account.owner == airdrop_wallet_state.key()
                     @ NovaPixelError::NotAdmin,
    )]
    pub airdrop_token_account: Account<'info, TokenAccount>,

    /// Player's NVPX token account (destination).
    #[account(
        mut,
        constraint = player_nvpx_account.owner == player.key()
                     @ NovaPixelError::NotAdmin,
    )]
    pub player_nvpx_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ClaimAirdrop>) -> Result<()> {
    require!(ctx.accounts.global_state.is_ended(), NovaPixelError::TournamentNotEnded);
    require!(!ctx.accounts.global_state.is_paused(), NovaPixelError::ContractPaused);

    let pa = &ctx.accounts.player_account;
    require!(!pa.airdrop_claimed, NovaPixelError::AirdropAlreadyClaimed);

    let total_pixels = ctx.accounts.global_state.total_correct_pixels;
    require!(total_pixels > 0, NovaPixelError::DivisionByZero);

    let player_pixels = pa.correct_pixels_colored;
    require!(player_pixels > 0, NovaPixelError::NoAirdropAllocation);

    // ── Compute share ─────────────────────────────────────────────────────────
    // share = (player_pixels / total_pixels) * AIRDROP_ALLOCATION
    // Use u128 to avoid overflow in the multiplication.
    let share = (player_pixels as u128)
        .checked_mul(AIRDROP_ALLOCATION as u128)
        .ok_or(NovaPixelError::MathOverflow)?
        .checked_div(total_pixels as u128)
        .ok_or(NovaPixelError::DivisionByZero)? as u64;

    require!(share > 0, NovaPixelError::NoAirdropAllocation);
    require!(
        ctx.accounts.airdrop_wallet_state.nvpx_balance >= share,
        NovaPixelError::AirdropPoolInsufficient
    );

    // ── Transfer airdrop tokens ───────────────────────────────────────────────
    let airdrop_bump = ctx.accounts.airdrop_wallet_state.bump;
    let signer_seeds: &[&[&[u8]]] = &[&[b"airdrop_wallet", &[airdrop_bump]]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from:      ctx.accounts.airdrop_token_account.to_account_info(),
                to:        ctx.accounts.player_nvpx_account.to_account_info(),
                authority: ctx.accounts.airdrop_wallet_state.to_account_info(),
            },
            signer_seeds,
        ),
        share,
    )?;

    // ── Update state ──────────────────────────────────────────────────────────
    ctx.accounts.player_account.airdrop_claimed   = true;
    ctx.accounts.player_account.airdrop_allocation = share;
    ctx.accounts.airdrop_wallet_state.nvpx_balance = ctx
        .accounts
        .airdrop_wallet_state
        .nvpx_balance
        .checked_sub(share)
        .ok_or(NovaPixelError::MathUnderflow)?;

    emit!(AirdropClaimed {
        wallet:    ctx.accounts.player.key(),
        amount:    share,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
