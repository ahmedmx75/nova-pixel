/// buy_item — purchase a Shield 3×3, Shield 5×5, or Rocket.
///
/// SOL paid goes directly to the buyback_wallet PDA — NOT to the airdrop pool.
/// The buyback_wallet SOL is later used by admin to buy NVPX from DEX when
/// the token price dips, creating transparent on-chain buy pressure.

use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer as SolTransfer};
use crate::{
    constants::*,
    errors::NovaPixelError,
    events::{ItemPurchased, RocketFired, ShieldActivated},
    state::{GlobalState, PlayerAccount, Shield, WalletState},
};

#[derive(Accounts)]
pub struct BuyItem<'info> {
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

    /// Buyback wallet PDA — receives the SOL.
    #[account(
        mut,
        seeds  = [b"buyback_wallet"],
        bump   = buyback_wallet_state.bump,
        constraint = buyback_wallet_state.key() == global_state.buyback_wallet
                     @ NovaPixelError::NotAdmin,
    )]
    pub buyback_wallet_state: Account<'info, WalletState>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx:       Context<BuyItem>,
    item_type: u8,
    // For shields: the center coordinates.  For rockets: the target coordinates.
    target_x:  u16,
    target_y:  u16,
    // For rockets: the defender whose shield should be destroyed.
    defender:  Option<Pubkey>,
) -> Result<()> {
    require!(ctx.accounts.global_state.is_active(), NovaPixelError::TournamentNotActive);
    require!(!ctx.accounts.global_state.is_paused(), NovaPixelError::ContractPaused);

    let sol_cost = match item_type {
        ITEM_SHIELD_3X3 => SHIELD_3X3_PRICE_LAMPORTS,
        ITEM_SHIELD_5X5 => SHIELD_5X5_PRICE_LAMPORTS,
        ITEM_ROCKET     => ROCKET_PRICE_LAMPORTS,
        _               => return err!(NovaPixelError::InvalidItemType),
    };

    // ── Transfer SOL: player → buyback_wallet PDA ─────────────────────────
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            SolTransfer {
                from: ctx.accounts.player.to_account_info(),
                to:   ctx.accounts.buyback_wallet_state.to_account_info(),
            },
        ),
        sol_cost,
    )?;

    let now = Clock::get()?.unix_timestamp;

    match item_type {
        // ── Shield items ──────────────────────────────────────────────────────
        ITEM_SHIELD_3X3 | ITEM_SHIELD_5X5 => {
            let size = if item_type == ITEM_SHIELD_3X3 {
                SHIELD_3X3_SIZE
            } else {
                SHIELD_5X5_SIZE
            };

            let pa = &mut ctx.accounts.player_account;
            let slot = pa
                .free_shield_slot(now)
                .ok_or(NovaPixelError::TooManyShields)?;

            pa.active_shields[slot] = Shield {
                center_x:    target_x,
                center_y:    target_y,
                size,
                expiry_time: now + SHIELD_DURATION_SECS,
            };

            emit!(ShieldActivated {
                wallet:   ctx.accounts.player.key(),
                team:     pa.team,
                center_x: target_x,
                center_y: target_y,
                size,
                expiry:   now + SHIELD_DURATION_SECS,
            });
        }

        // ── Rocket — destroys one enemy shield ────────────────────────────────
        ITEM_ROCKET => {
            // The client passes the defender pubkey; rocket resolves immediately.
            // The actual removal of the shield is done via a follow-up
            // `rocket_resolve` call by the game server, which has access to the
            // defender's account.  Here we just emit the event and record cost.
            // (See rocket_resolve in admin.rs for the shield-removal CPI.)
            emit!(RocketFired {
                wallet:           ctx.accounts.player.key(),
                target_x,
                target_y,
                shield_destroyed: defender.is_some(), // game server confirms later
            });
        }

        _ => unreachable!(),
    }

    emit!(ItemPurchased {
        wallet:    ctx.accounts.player.key(),
        item_type,
        sol_cost,
    });

    Ok(())
}
