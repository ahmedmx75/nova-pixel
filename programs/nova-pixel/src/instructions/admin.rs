/// Admin functions — not tournament controls.
///
/// burn_tokens:     one-time burn of the burn_wallet allocation (Phase 04).
/// admin_buyback:   use SOL from buyback_wallet to purchase NVPX via Jupiter.
/// rocket_resolve:  game-server helper to remove a specific shield (after rocket).
/// withdraw_locked: admin withdraws from a time-unlocked wallet (dev/reserve only).

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
use crate::{
    errors::NovaPixelError,
    events::{BuybackExecuted, TokensBurned},
    jupiter,
    state::{GlobalState, PlayerAccount, WalletState, WalletType},
};

// ── burn_tokens ───────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct BurnTokens<'info> {
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"global_state"],
        bump  = global_state.bump,
        constraint = admin.key() == global_state.admin @ NovaPixelError::NotAdmin,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds  = [b"burn_wallet"],
        bump   = burn_wallet_state.bump,
        constraint = !burn_wallet_state.is_burned @ NovaPixelError::AlreadyBurned,
    )]
    pub burn_wallet_state: Account<'info, WalletState>,

    #[account(
        mut,
        constraint = burn_token_account.owner == burn_wallet_state.key()
                     @ NovaPixelError::NotAdmin,
    )]
    pub burn_token_account: Account<'info, TokenAccount>,

    #[account(mut, constraint = nvpx_mint.key() == global_state.nvpx_mint)]
    pub nvpx_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

pub fn burn_handler(ctx: Context<BurnTokens>) -> Result<()> {
    let amount     = ctx.accounts.burn_wallet_state.nvpx_balance;
    let burn_bump  = ctx.accounts.burn_wallet_state.bump;
    let signer_seeds: &[&[&[u8]]] = &[&[b"burn_wallet", &[burn_bump]]];

    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint:      ctx.accounts.nvpx_mint.to_account_info(),
                from:      ctx.accounts.burn_token_account.to_account_info(),
                authority: ctx.accounts.burn_wallet_state.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    ctx.accounts.burn_wallet_state.nvpx_balance = 0;
    ctx.accounts.burn_wallet_state.is_burned    = true;

    emit!(TokensBurned {
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

// ── admin_buyback ─────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct AdminBuyback<'info> {
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"global_state"],
        bump  = global_state.bump,
        constraint = admin.key() == global_state.admin @ NovaPixelError::NotAdmin,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds  = [b"buyback_wallet"],
        bump   = buyback_wallet_state.bump,
    )]
    pub buyback_wallet_state: Account<'info, WalletState>,

    /// Destination for bought-back NVPX (reserve or liquidity wallet ATA).
    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,

    /// CHECK: validated inside jupiter::cpi_swap.
    pub jupiter_program: AccountInfo<'info>,
}

pub fn buyback_handler(
    ctx:          Context<AdminBuyback>,
    sol_amount:   u64,
    jupiter_data: Vec<u8>,
) -> Result<()> {
    require!(!ctx.accounts.global_state.is_paused(), NovaPixelError::ContractPaused);

    let nvpx_before = ctx.accounts.destination_token_account.amount;

    let buyback_bump = ctx.accounts.buyback_wallet_state.bump;
    let signer_seeds: &[&[&[u8]]] = &[&[b"buyback_wallet", &[buyback_bump]]];

    jupiter::cpi_swap(
        ctx.remaining_accounts,
        jupiter_data,
        signer_seeds,
    )?;

    ctx.accounts.destination_token_account.reload()?;
    let nvpx_received = ctx
        .accounts
        .destination_token_account
        .amount
        .checked_sub(nvpx_before)
        .ok_or(NovaPixelError::MathUnderflow)?;

    require!(nvpx_received > 0, NovaPixelError::ZeroTokensReceived);

    emit!(BuybackExecuted {
        sol_spent:   sol_amount,
        nvpx_bought: nvpx_received,
        timestamp:   Clock::get()?.unix_timestamp,
    });

    Ok(())
}

// ── rocket_resolve ────────────────────────────────────────────────────────────
/// Called by game server after a Rocket purchase to actually remove the
/// defender's shield.  Separated from buy_item because the defender's
/// account is a different PDA that buy_item can't directly mutate.
#[derive(Accounts)]
#[instruction(defender_wallet: Pubkey)]
pub struct RocketResolve<'info> {
    pub game_server: Signer<'info>,

    #[account(
        seeds = [b"global_state"],
        bump  = global_state.bump,
        constraint = game_server.key() == global_state.game_server
                     @ NovaPixelError::NotGameServer,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds  = [b"player", defender_wallet.as_ref()],
        bump   = defender_account.bump,
        constraint = defender_account.is_initialized @ NovaPixelError::PlayerNotInitialized,
    )]
    pub defender_account: Account<'info, PlayerAccount>,
}

pub fn rocket_resolve_handler(
    ctx:             Context<RocketResolve>,
    _defender_wallet: Pubkey,
    target_x:        u16,
    target_y:        u16,
) -> Result<()> {
    require!(ctx.accounts.global_state.is_active(), NovaPixelError::TournamentNotActive);
    let now = Clock::get()?.unix_timestamp;

    let removed = ctx
        .accounts
        .defender_account
        .remove_shield_at(target_x, target_y, now);

    require!(removed, NovaPixelError::NoShieldAtTarget);
    Ok(())
}

// ── withdraw_locked ───────────────────────────────────────────────────────────
/// Admin withdraws from a time-unlocked wallet (development or reserve only).
/// Liquidity, airdrop, team, and burn wallets are explicitly excluded.
#[derive(Accounts)]
pub struct WithdrawLocked<'info> {
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"global_state"],
        bump  = global_state.bump,
        constraint = admin.key() == global_state.admin @ NovaPixelError::NotAdmin,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut)]
    pub wallet_state: Account<'info, WalletState>,

    #[account(
        mut,
        constraint = source_token_account.owner == wallet_state.key()
                     @ NovaPixelError::NotAdmin,
    )]
    pub source_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw_locked_handler(ctx: Context<WithdrawLocked>, amount: u64) -> Result<()> {
    let ws = &ctx.accounts.wallet_state;

    // Only development and reserve wallets may be admin-withdrawn
    require!(
        ws.wallet_type == WalletType::Development || ws.wallet_type == WalletType::Reserve,
        NovaPixelError::NotAdmin
    );

    // Enforce time lock
    let now = Clock::get()?.unix_timestamp;
    require!(ws.is_time_unlocked(now), NovaPixelError::WalletLocked);

    require!(ws.nvpx_balance >= amount, NovaPixelError::InsufficientBalance);

    // Determine correct PDA seed for signer
    let seed: &[u8] = match ws.wallet_type {
        WalletType::Development => b"development_wallet",
        WalletType::Reserve     => b"reserve_wallet",
        _                       => return err!(NovaPixelError::NotAdmin),
    };
    let bump = ws.bump;
    let signer_seeds: &[&[&[u8]]] = &[&[seed, &[bump]]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from:      ctx.accounts.source_token_account.to_account_info(),
                to:        ctx.accounts.destination_token_account.to_account_info(),
                authority: ctx.accounts.wallet_state.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    ctx.accounts.wallet_state.nvpx_balance = ctx
        .accounts
        .wallet_state
        .nvpx_balance
        .checked_sub(amount)
        .ok_or(NovaPixelError::MathUnderflow)?;

    Ok(())
}
