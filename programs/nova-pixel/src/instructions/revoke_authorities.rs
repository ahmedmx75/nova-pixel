/// revoke_authorities — permanently revoke the NVPX mint authority and
/// freeze authority so the token becomes fully immutable.
///
/// WHY THIS MATTERS
/// ────────────────
/// CEX listings (Binance, OKX, Bybit, etc.) and token-tracking sites
/// (CoinMarketCap, CoinGecko) require that a token's mint authority and
/// freeze authority are set to `None` before listing approval.
///
///   • Mint authority = None  → supply is permanently capped at 1,000,000,000 NVPX.
///   • Freeze authority = None → fully non-custodial; no accounts can ever be frozen.
///
/// Both show as "Immutable" on Solscan / SolanaFM / Birdeye after this call.
///
/// WHEN TO CALL
/// ────────────
/// Call ONCE, after the full 1,000,000,000 NVPX supply has been minted
/// to all wallet PDAs (i.e., after deploy/migrate script finishes).
/// This is an irreversible, one-way operation — there is no undo.
///
/// # Security
///   - Admin must be the current mint authority and freeze authority.
///   - Guarded by `authorities_revoked` flag in GlobalState (cannot call twice).
///   - Calls SPL Token `set_authority` with `new_authority = None` for both types.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, spl_token, Mint, SetAuthority, Token};
use spl_token::instruction::AuthorityType;
use crate::{
    errors::NovaPixelError,
    events::MintAuthorityRevoked,
    state::GlobalState,
};

#[derive(Accounts)]
pub struct RevokeAuthorities<'info> {
    /// Must be the current mint authority AND freeze authority.
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global_state"],
        bump  = global_state.bump,
        constraint = admin.key() == global_state.admin @ NovaPixelError::NotAdmin,
        constraint = !global_state.authorities_revoked @ NovaPixelError::AuthorityAlreadyRevoked,
    )]
    pub global_state: Account<'info, GlobalState>,

    /// The NVPX mint — admin must be the current mint authority.
    #[account(
        mut,
        constraint = nvpx_mint.key() == global_state.nvpx_mint @ NovaPixelError::NotAdmin,
    )]
    pub nvpx_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<RevokeAuthorities>) -> Result<()> {
    // ── Revoke mint authority ──────────────────────────────────────────────────
    // Supply is permanently capped at 1,000,000,000 NVPX after this call.
    token::set_authority(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SetAuthority {
                account_or_mint: ctx.accounts.nvpx_mint.to_account_info(),
                current_authority: ctx.accounts.admin.to_account_info(),
            },
        ),
        AuthorityType::MintTokens,
        None, // ← permanently removes mint authority
    )?;

    // ── Revoke freeze authority ────────────────────────────────────────────────
    // Only call if freeze authority is still set — if it's already None the
    // SPL Token program returns MintCannotFreeze (0x10) and the tx fails.
    if ctx.accounts.nvpx_mint.freeze_authority.is_some() {
        token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    account_or_mint: ctx.accounts.nvpx_mint.to_account_info(),
                    current_authority: ctx.accounts.admin.to_account_info(),
                },
            ),
            AuthorityType::FreezeAccount,
            None, // ← permanently removes freeze authority
        )?;
    }

    // ── Mark as done in global state ───────────────────────────────────────────
    ctx.accounts.global_state.authorities_revoked = true;

    emit!(MintAuthorityRevoked {
        mint:      ctx.accounts.nvpx_mint.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!(
        "NVPX mint authority and freeze authority permanently revoked. \
         Token is immutable — supply fixed at 1,000,000,000 NVPX. \
         Visible as immutable on Solscan/SolanaFM/Birdeye."
    );

    Ok(())
}
