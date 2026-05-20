use anchor_lang::prelude::*;
use crate::{
    constants::{MAX_SHIELDS, MAX_TEAMS},
    errors::NovaPixelError,
    events::PlayerConnected,
    state::{GlobalState, PlayerAccount, Shield},
};

#[derive(Accounts)]
pub struct ConnectPlayer<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(seeds = [b"global_state"], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        init,
        payer  = player,
        space  = PlayerAccount::LEN,
        seeds  = [b"player", player.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ConnectPlayer>, team: u8) -> Result<()> {
    require!(
        !ctx.accounts.global_state.is_paused(),
        NovaPixelError::ContractPaused
    );
    require!(team < MAX_TEAMS, NovaPixelError::InvalidTeam);

    let now = Clock::get()?.unix_timestamp;
    let pa  = &mut ctx.accounts.player_account;

    pa.wallet_address         = ctx.accounts.player.key();
    pa.team                   = team;
    pa.attempts_balance       = 0;
    pa.in_game_nvpx_balance   = 0;
    pa.correct_pixels_colored = 0;
    pa.airdrop_allocation     = 0;
    pa.airdrop_claimed        = false;
    pa.join_timestamp         = now;
    pa.total_sol_spent        = 0;
    pa.active_shields         = [Shield::default(); MAX_SHIELDS];
    pa.is_initialized         = true;
    pa.bump                   = ctx.bumps.player_account;

    emit!(PlayerConnected {
        wallet:    ctx.accounts.player.key(),
        team,
        timestamp: now,
    });

    Ok(())
}
