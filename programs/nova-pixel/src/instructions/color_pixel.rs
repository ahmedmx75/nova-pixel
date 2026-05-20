/// color_pixel — called by the game server (oracle) when a player correctly
/// colors a pixel on the off-chain canvas.
///
/// Grants the player 2× the pixel's NVPX value as an in-game balance,
/// drawing from the airdrop pool.  Also records the pixel toward the
/// player's airdrop allocation and the global pixel count.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    constants::PIXEL_REWARD_MULTIPLIER,
    errors::NovaPixelError,
    events::PixelColored,
    state::{GlobalState, PlayerAccount, WalletState},
};

#[derive(Accounts)]
#[instruction(player_wallet: Pubkey)]
pub struct ColorPixel<'info> {
    /// Game server — trusted oracle that reports canvas events on-chain.
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
        seeds  = [b"player", player_wallet.as_ref()],
        bump   = player_account.bump,
        constraint = player_account.is_initialized @ NovaPixelError::PlayerNotInitialized,
    )]
    pub player_account: Account<'info, PlayerAccount>,

    /// Airdrop wallet state — the source of 2× reward funds.
    #[account(
        mut,
        seeds  = [b"airdrop_wallet"],
        bump   = airdrop_wallet_state.bump,
    )]
    pub airdrop_wallet_state: Account<'info, WalletState>,

    /// NVPX token account owned by airdrop_wallet_state (source of transfer).
    #[account(
        mut,
        constraint = airdrop_token_account.owner == airdrop_wallet_state.key()
                     @ NovaPixelError::NotAdmin,
    )]
    pub airdrop_token_account: Account<'info, TokenAccount>,

    /// Player's NVPX token account (destination of reward transfer).
    #[account(
        mut,
        constraint = player_nvpx_account.owner == player_wallet
                     @ NovaPixelError::NotAdmin,
    )]
    pub player_nvpx_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx:           Context<ColorPixel>,
    player_wallet: Pubkey,
    x:             u16,
    y:             u16,
    pixel_value:   u64, // base NVPX value of this pixel (game server determines this)
    is_correct:    bool,
) -> Result<()> {
    require!(ctx.accounts.global_state.is_active(), NovaPixelError::TournamentNotActive);
    require!(!ctx.accounts.global_state.is_paused(), NovaPixelError::ContractPaused);
    require!(pixel_value > 0, NovaPixelError::ZeroPixelValue);

    let reward = if is_correct {
        pixel_value
            .checked_mul(PIXEL_REWARD_MULTIPLIER)
            .ok_or(NovaPixelError::MathOverflow)?
    } else {
        0
    };

    if reward > 0 {
        // Verify pool has enough
        require!(
            ctx.accounts.airdrop_wallet_state.nvpx_balance >= reward,
            NovaPixelError::AirdropPoolInsufficient
        );

        // Transfer reward from airdrop_wallet ATA → player ATA
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
            reward,
        )?;

        // ── Update state ──────────────────────────────────────────────────────
        let gs = &mut ctx.accounts.global_state;
        gs.total_nvpx_rewarded = gs
            .total_nvpx_rewarded
            .checked_add(reward)
            .ok_or(NovaPixelError::MathOverflow)?;
        gs.total_nvpx_in_game = gs
            .total_nvpx_in_game
            .checked_add(reward)
            .ok_or(NovaPixelError::MathOverflow)?;

        let aws = &mut ctx.accounts.airdrop_wallet_state;
        aws.nvpx_balance = aws
            .nvpx_balance
            .checked_sub(reward)
            .ok_or(NovaPixelError::MathUnderflow)?;

        let pa = &mut ctx.accounts.player_account;
        pa.in_game_nvpx_balance = pa
            .in_game_nvpx_balance
            .checked_add(reward)
            .ok_or(NovaPixelError::MathOverflow)?;
        pa.correct_pixels_colored = pa
            .correct_pixels_colored
            .checked_add(1)
            .ok_or(NovaPixelError::MathOverflow)?;
        // Airdrop share grows proportionally with correct pixels (formula
        // is finalized in end_tournament / claim_airdrop).
        pa.airdrop_allocation = pa
            .airdrop_allocation
            .checked_add(pixel_value)
            .ok_or(NovaPixelError::MathOverflow)?;

        ctx.accounts.global_state.total_correct_pixels = ctx
            .accounts
            .global_state
            .total_correct_pixels
            .checked_add(1)
            .ok_or(NovaPixelError::MathOverflow)?;
    }

    // Deduct one attempt
    let pa = &mut ctx.accounts.player_account;
    require!(pa.attempts_balance > 0, NovaPixelError::InsufficientAttempts);
    pa.attempts_balance -= 1;

    emit!(PixelColored {
        wallet:      player_wallet,
        team:        ctx.accounts.player_account.team,
        x,
        y,
        is_correct,
        nvpx_reward: reward,
    });

    Ok(())
}
