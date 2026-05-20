/// capture_pixel — called by the game server when an attacker successfully
/// colors over a pixel that a defender previously earned a 2× reward on.
///
/// The defender loses their in-game reward for that pixel; the attacker
/// gains it.  If the pixel is shielded, the capture is blocked.

use anchor_lang::prelude::*;
use crate::{
    errors::NovaPixelError,
    events::PixelCaptured,
    state::{GlobalState, PlayerAccount},
};

#[derive(Accounts)]
#[instruction(attacker_wallet: Pubkey, defender_wallet: Pubkey)]
pub struct CapturePixel<'info> {
    pub game_server: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global_state"],
        bump  = global_state.bump,
        constraint = game_server.key() == global_state.game_server
                     @ NovaPixelError::NotGameServer,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds  = [b"player", attacker_wallet.as_ref()],
        bump   = attacker_account.bump,
        constraint = attacker_account.is_initialized @ NovaPixelError::PlayerNotInitialized,
    )]
    pub attacker_account: Account<'info, PlayerAccount>,

    #[account(
        mut,
        seeds  = [b"player", defender_wallet.as_ref()],
        bump   = defender_account.bump,
        constraint = defender_account.is_initialized @ NovaPixelError::PlayerNotInitialized,
    )]
    pub defender_account: Account<'info, PlayerAccount>,
}

pub fn handler(
    ctx:             Context<CapturePixel>,
    attacker_wallet: Pubkey,
    defender_wallet: Pubkey,
    x:               u16,
    y:               u16,
    nvpx_reward:     u64, // the 2× reward originally earned for this pixel
) -> Result<()> {
    require!(ctx.accounts.global_state.is_active(), NovaPixelError::TournamentNotActive);
    require!(!ctx.accounts.global_state.is_paused(), NovaPixelError::ContractPaused);
    require!(nvpx_reward > 0, NovaPixelError::ZeroPixelValue);

    let now = Clock::get()?.unix_timestamp;

    // ── Shield check — if defender has a shield covering this pixel, block ──
    let defender_shielded = ctx
        .accounts
        .defender_account
        .has_active_shield_at(x, y, now);

    if defender_shielded {
        return err!(NovaPixelError::PixelShielded);
    }

    // ── Transfer in-game balance: defender → attacker ─────────────────────
    let defender_balance = ctx.accounts.defender_account.in_game_nvpx_balance;
    let transfer_amount  = nvpx_reward.min(defender_balance); // never take more than they have

    ctx.accounts.defender_account.in_game_nvpx_balance = defender_balance
        .checked_sub(transfer_amount)
        .ok_or(NovaPixelError::MathUnderflow)?;

    ctx.accounts.attacker_account.in_game_nvpx_balance = ctx
        .accounts
        .attacker_account
        .in_game_nvpx_balance
        .checked_add(transfer_amount)
        .ok_or(NovaPixelError::MathOverflow)?;

    // Defender loses the airdrop points accumulated for this pixel.
    // Attacker gains 1 correct pixel (game server already validated the coloring).
    ctx.accounts.defender_account.correct_pixels_colored = ctx
        .accounts
        .defender_account
        .correct_pixels_colored
        .saturating_sub(1);

    ctx.accounts.attacker_account.correct_pixels_colored = ctx
        .accounts
        .attacker_account
        .correct_pixels_colored
        .checked_add(1)
        .ok_or(NovaPixelError::MathOverflow)?;

    emit!(PixelCaptured {
        attacker_wallet,
        defender_wallet,
        x,
        y,
        nvpx_transferred: transfer_amount,
    });

    Ok(())
}
