/// initialize — two-step setup to stay within the 4 KB SBF stack-frame limit.
///
/// Step 1 — `initialize`:
///   Creates global_state PDA and all 7 wallet state PDAs.
///   Admin must call this first with params (game_server, multisig, timelocks).
///
/// Step 2 — `initialize_token_accounts`:
///   Creates the 6 NVPX ATAs owned by the wallet PDAs.
///   Must be called right after step 1 (global_state must already exist).
///
/// NOTE: The NVPX mint must be pre-created (admin creates it separately,
/// mints the full 1B supply, then calls initialize → initialize_token_accounts
/// → revoke_authorities).

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use crate::{
    constants::*,
    state::*,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeParams {
    pub game_server:          Pubkey,
    pub multisig_signers:     [Pubkey; 3],
    pub use_devnet_timelocks: bool,
}

// ── Step 1: global_state + 7 wallet PDAs ─────────────────────────────────────
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer  = admin,
        space  = GlobalState::LEN,
        seeds  = [b"global_state"],
        bump
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    pub nvpx_mint: Box<Account<'info, Mint>>,

    #[account(init, payer = admin, space = WalletState::LEN,
              seeds = [b"liquidity_wallet"],   bump)]
    pub liquidity_wallet_state:   Box<Account<'info, WalletState>>,

    #[account(init, payer = admin, space = WalletState::LEN,
              seeds = [b"airdrop_wallet"],     bump)]
    pub airdrop_wallet_state:     Box<Account<'info, WalletState>>,

    #[account(init, payer = admin, space = WalletState::LEN,
              seeds = [b"team_wallet"],        bump)]
    pub team_wallet_state:        Box<Account<'info, WalletState>>,

    #[account(init, payer = admin, space = WalletState::LEN,
              seeds = [b"development_wallet"], bump)]
    pub development_wallet_state: Box<Account<'info, WalletState>>,

    #[account(init, payer = admin, space = WalletState::LEN,
              seeds = [b"burn_wallet"],        bump)]
    pub burn_wallet_state:        Box<Account<'info, WalletState>>,

    #[account(init, payer = admin, space = WalletState::LEN,
              seeds = [b"reserve_wallet"],     bump)]
    pub reserve_wallet_state:     Box<Account<'info, WalletState>>,

    #[account(init, payer = admin, space = WalletState::LEN,
              seeds = [b"buyback_wallet"],     bump)]
    pub buyback_wallet_state:     Box<Account<'info, WalletState>>,

    pub system_program: Program<'info, System>,
    pub rent:           Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;

    let liq_lock = if params.use_devnet_timelocks {
        now + DEVNET_LOCK_DURATION
    } else {
        now + LIQUIDITY_LOCK_DURATION
    };
    let team_lock = if params.use_devnet_timelocks {
        now + DEVNET_LOCK_DURATION
    } else {
        now + TEAM_LOCK_DURATION
    };
    let dev_lock = if params.use_devnet_timelocks {
        now + DEVNET_LOCK_DURATION
    } else {
        now + DEVELOPMENT_LOCK_DURATION
    };

    // ── GlobalState ───────────────────────────────────────────────────────────
    let gs = &mut ctx.accounts.global_state;
    gs.admin             = ctx.accounts.admin.key();
    gs.game_server       = params.game_server;
    gs.multisig_signers  = params.multisig_signers;
    gs.pause_votes       = [false; 3];
    gs.tournament_state      = TournamentState::Inactive;
    gs.current_round         = 0;
    gs.airdrop_finalized     = false;
    gs.authorities_revoked   = false;
    gs.nvpx_mint             = ctx.accounts.nvpx_mint.key();
    gs.total_correct_pixels  = 0;
    gs.total_nvpx_in_game    = 0;
    gs.total_nvpx_rewarded   = 0;
    gs.tournament_start_time = 0;
    gs.tournament_end_time   = 0;
    gs.initialized_at        = now;
    gs.liquidity_wallet      = ctx.accounts.liquidity_wallet_state.key();
    gs.airdrop_wallet        = ctx.accounts.airdrop_wallet_state.key();
    gs.team_wallet           = ctx.accounts.team_wallet_state.key();
    gs.development_wallet    = ctx.accounts.development_wallet_state.key();
    gs.burn_wallet           = ctx.accounts.burn_wallet_state.key();
    gs.reserve_wallet        = ctx.accounts.reserve_wallet_state.key();
    gs.buyback_wallet        = ctx.accounts.buyback_wallet_state.key();
    gs.bump                  = ctx.bumps.global_state;

    // ── Wallet states ─────────────────────────────────────────────────────────
    macro_rules! init_ws {
        ($ws:expr, $wtype:expr, $alloc:expr, $lock:expr, $bump:expr) => {{
            let ws = &mut $ws;
            ws.wallet_type        = $wtype;
            ws.nvpx_balance       = $alloc;
            ws.initial_allocation = $alloc;
            ws.lock_until         = $lock;
            ws.is_burned          = false;
            ws.bump               = $bump;
        }};
    }

    init_ws!(ctx.accounts.liquidity_wallet_state,   WalletType::Liquidity,   LIQUIDITY_ALLOCATION,   liq_lock,    ctx.bumps.liquidity_wallet_state);
    init_ws!(ctx.accounts.airdrop_wallet_state,     WalletType::Airdrop,     AIRDROP_ALLOCATION,     i64::MAX,    ctx.bumps.airdrop_wallet_state);
    init_ws!(ctx.accounts.team_wallet_state,        WalletType::Team,        TEAM_ALLOCATION,        team_lock,   ctx.bumps.team_wallet_state);
    init_ws!(ctx.accounts.development_wallet_state, WalletType::Development, DEVELOPMENT_ALLOCATION, dev_lock,    ctx.bumps.development_wallet_state);
    init_ws!(ctx.accounts.burn_wallet_state,        WalletType::Burn,        BURN_ALLOCATION,        0,           ctx.bumps.burn_wallet_state);
    init_ws!(ctx.accounts.reserve_wallet_state,     WalletType::Reserve,     RESERVE_ALLOCATION,     0,           ctx.bumps.reserve_wallet_state);
    // buyback receives SOL, starts with zero NVPX balance
    init_ws!(ctx.accounts.buyback_wallet_state,     WalletType::Buyback,     0,                      0,           ctx.bumps.buyback_wallet_state);

    Ok(())
}

// ── Step 2: create the 6 NVPX ATAs ───────────────────────────────────────────
#[derive(Accounts)]
pub struct InitializeTokenAccounts<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    // global_state must already exist
    #[account(seeds = [b"global_state"], bump = global_state.bump)]
    pub global_state: Box<Account<'info, GlobalState>>,

    pub nvpx_mint: Box<Account<'info, Mint>>,

    // Wallet PDAs — already created in step 1
    #[account(seeds = [b"liquidity_wallet"],   bump = liquidity_wallet_state.bump)]
    pub liquidity_wallet_state:   Box<Account<'info, WalletState>>,
    #[account(seeds = [b"airdrop_wallet"],     bump = airdrop_wallet_state.bump)]
    pub airdrop_wallet_state:     Box<Account<'info, WalletState>>,
    #[account(seeds = [b"team_wallet"],        bump = team_wallet_state.bump)]
    pub team_wallet_state:        Box<Account<'info, WalletState>>,
    #[account(seeds = [b"development_wallet"], bump = development_wallet_state.bump)]
    pub development_wallet_state: Box<Account<'info, WalletState>>,
    #[account(seeds = [b"burn_wallet"],        bump = burn_wallet_state.bump)]
    pub burn_wallet_state:        Box<Account<'info, WalletState>>,
    #[account(seeds = [b"reserve_wallet"],     bump = reserve_wallet_state.bump)]
    pub reserve_wallet_state:     Box<Account<'info, WalletState>>,

    // Token ATAs — created here (init)
    #[account(init, payer = admin,
              associated_token::mint      = nvpx_mint,
              associated_token::authority = liquidity_wallet_state)]
    pub liquidity_token_account:   Box<Account<'info, TokenAccount>>,

    #[account(init, payer = admin,
              associated_token::mint      = nvpx_mint,
              associated_token::authority = airdrop_wallet_state)]
    pub airdrop_token_account:     Box<Account<'info, TokenAccount>>,

    #[account(init, payer = admin,
              associated_token::mint      = nvpx_mint,
              associated_token::authority = team_wallet_state)]
    pub team_token_account:        Box<Account<'info, TokenAccount>>,

    #[account(init, payer = admin,
              associated_token::mint      = nvpx_mint,
              associated_token::authority = development_wallet_state)]
    pub development_token_account: Box<Account<'info, TokenAccount>>,

    #[account(init, payer = admin,
              associated_token::mint      = nvpx_mint,
              associated_token::authority = burn_wallet_state)]
    pub burn_token_account:        Box<Account<'info, TokenAccount>>,

    #[account(init, payer = admin,
              associated_token::mint      = nvpx_mint,
              associated_token::authority = reserve_wallet_state)]
    pub reserve_token_account:     Box<Account<'info, TokenAccount>>,

    // buyback_wallet holds SOL only — no token account needed

    pub token_program:            Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program:           Program<'info, System>,
    pub rent:                     Sysvar<'info, Rent>,
}

pub fn initialize_token_accounts_handler(
    _ctx: Context<InitializeTokenAccounts>,
) -> Result<()> {
    // All setup is done by the Anchor constraints (init ATAs).
    // No additional logic needed here.
    Ok(())
}
