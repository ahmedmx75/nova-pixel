/// sell_ingame — player sells in-game NVPX during an active tournament.
///
/// Penalty breakdown (active tournament only):
///   50% of amount → back to airdrop pool
///   2% tax on remaining 50% → development_wallet
///   Player receives the rest (~48% of original amount)
///
/// Example: sell 2,000 NVPX
///   penalty   = 1,000 NVPX → pool
///   remainder = 1,000 NVPX
///   tax       =    20 NVPX → development
///   received  =   980 NVPX → player
///
/// Player also permanently loses airdrop share for sold pixels.
///
/// NOTE: This applies ONLY to in-game sells via the contract.
///       Direct DEX trades are unaffected.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{
    constants::*,
    errors::NovaPixelError,
    events::InGameSell,
    state::{GlobalState, PlayerAccount, WalletState},
};

#[derive(Accounts)]
pub struct SellIngame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds  = [b"player", player.key().as_ref()],
        bump   = player_account.bump,
        constraint = player_account.is_initialized @ NovaPixelError::PlayerNotInitialized,
    )]
    pub player_account: Account<'info, PlayerAccount>,

    #[account(mut, seeds = [b"global_state"], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,

    /// Airdrop wallet state — receives the 50% penalty.
    #[account(
        mut,
        seeds  = [b"airdrop_wallet"],
        bump   = airdrop_wallet_state.bump,
    )]
    pub airdrop_wallet_state: Account<'info, WalletState>,

    /// Development wallet state — receives the 2% tax.
    #[account(
        mut,
        seeds  = [b"development_wallet"],
        bump   = development_wallet_state.bump,
    )]
    pub development_wallet_state: Account<'info, WalletState>,

    /// Player's NVPX token account (source — their in-game holdings).
    #[account(
        mut,
        constraint = player_nvpx_account.owner == player.key()
                     @ NovaPixelError::NotAdmin,
    )]
    pub player_nvpx_account: Account<'info, TokenAccount>,

    /// Airdrop pool token account (destination for penalty).
    #[account(
        mut,
        constraint = airdrop_token_account.owner == airdrop_wallet_state.key()
                     @ NovaPixelError::NotAdmin,
    )]
    pub airdrop_token_account: Account<'info, TokenAccount>,

    /// Development wallet token account (destination for tax).
    #[account(
        mut,
        constraint = development_token_account.owner == development_wallet_state.key()
                     @ NovaPixelError::NotAdmin,
    )]
    pub development_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<SellIngame>, amount: u64) -> Result<()> {
    require!(ctx.accounts.global_state.is_active(), NovaPixelError::TournamentNotActive);
    require!(!ctx.accounts.global_state.is_paused(), NovaPixelError::ContractPaused);
    require!(amount > 0, NovaPixelError::InvalidSellAmount);

    let pa = &ctx.accounts.player_account;
    require!(
        pa.in_game_nvpx_balance >= amount,
        NovaPixelError::InsufficientBalance
    );

    // ── Penalty math (all checked arithmetic) ────────────────────────────────
    //   penalty = amount * 50%
    //   tax     = (amount - penalty) * 2%
    //   received = amount - penalty - tax
    let penalty = amount
        .checked_mul(SELL_PENALTY_BPS)
        .ok_or(NovaPixelError::MathOverflow)?
        .checked_div(BPS_DENOMINATOR)
        .ok_or(NovaPixelError::DivisionByZero)?;

    let remainder = amount
        .checked_sub(penalty)
        .ok_or(NovaPixelError::MathUnderflow)?;

    let tax = remainder
        .checked_mul(SELL_TAX_BPS)
        .ok_or(NovaPixelError::MathOverflow)?
        .checked_div(BPS_DENOMINATOR)
        .ok_or(NovaPixelError::DivisionByZero)?;

    let received = remainder
        .checked_sub(tax)
        .ok_or(NovaPixelError::MathUnderflow)?;

    // ── Transfer penalty: player → airdrop pool ───────────────────────────
    if penalty > 0 {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from:      ctx.accounts.player_nvpx_account.to_account_info(),
                    to:        ctx.accounts.airdrop_token_account.to_account_info(),
                    authority: ctx.accounts.player.to_account_info(),
                },
            ),
            penalty,
        )?;
        ctx.accounts.airdrop_wallet_state.nvpx_balance = ctx
            .accounts
            .airdrop_wallet_state
            .nvpx_balance
            .checked_add(penalty)
            .ok_or(NovaPixelError::MathOverflow)?;
    }

    // ── Transfer tax: player → development wallet ─────────────────────────
    if tax > 0 {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from:      ctx.accounts.player_nvpx_account.to_account_info(),
                    to:        ctx.accounts.development_token_account.to_account_info(),
                    authority: ctx.accounts.player.to_account_info(),
                },
            ),
            tax,
        )?;
        ctx.accounts.development_wallet_state.nvpx_balance = ctx
            .accounts
            .development_wallet_state
            .nvpx_balance
            .checked_add(tax)
            .ok_or(NovaPixelError::MathOverflow)?;
    }

    // ── Update player state ───────────────────────────────────────────────
    let pa = &mut ctx.accounts.player_account;
    pa.in_game_nvpx_balance = pa
        .in_game_nvpx_balance
        .checked_sub(amount)
        .ok_or(NovaPixelError::MathUnderflow)?;

    // Proportionally reduce airdrop allocation
    let alloc_to_lose = pa
        .airdrop_allocation
        .checked_mul(amount)
        .ok_or(NovaPixelError::MathOverflow)?
        .checked_div(pa.in_game_nvpx_balance.saturating_add(amount))
        .unwrap_or(0);
    pa.airdrop_allocation = pa.airdrop_allocation.saturating_sub(alloc_to_lose);

    // Reflect the overall in-game total
    ctx.accounts.global_state.total_nvpx_in_game = ctx
        .accounts
        .global_state
        .total_nvpx_in_game
        .saturating_sub(amount);

    emit!(InGameSell {
        wallet:          ctx.accounts.player.key(),
        nvpx_sold:       amount,
        penalty_amount:  penalty,
        tax_amount:      tax,
        received_amount: received,
    });

    Ok(())
}
