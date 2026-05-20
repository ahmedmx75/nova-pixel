/// buy_package — purchase a Starter / Advanced / Pro package.
///
/// Flow:
///   1. Player sends SOL to the program's sol_vault PDA.
///   2. The sol_vault PDA signs a Jupiter CPI to swap SOL → NVPX.
///   3. Jupiter deposits NVPX directly into the airdrop_wallet's token account.
///   4. Contract records player attempts and emits PackagePurchased.
///
/// Jupiter swap data (route bytes + required accounts) must be obtained
/// off-chain from the Jupiter /quote + /swap-instructions API and passed in
/// as `jupiter_data` / `ctx.remaining_accounts`.

use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer as SolTransfer};
use anchor_spl::token::TokenAccount;
use crate::{
    constants::*,
    errors::NovaPixelError,
    events::PackagePurchased,
    jupiter,
    state::{GlobalState, PlayerAccount, WalletState},
};

#[derive(Accounts)]
pub struct BuyPackage<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds = [b"player", player.key().as_ref()],
        bump  = player_account.bump,
        constraint = player_account.is_initialized @ NovaPixelError::PlayerNotInitialized,
    )]
    pub player_account: Account<'info, PlayerAccount>,

    #[account(mut, seeds = [b"global_state"], bump = global_state.bump)]
    pub global_state: Account<'info, GlobalState>,

    /// SOL vault PDA — receives SOL from player then signs the Jupiter CPI.
    /// CHECK: PDA derived with seeds [b"sol_vault"]; validated by seed constraint.
    #[account(mut, seeds = [b"sol_vault"], bump)]
    pub sol_vault: SystemAccount<'info>,

    /// Airdrop wallet state — tracks in-pool NVPX balance.
    #[account(
        mut,
        seeds  = [b"airdrop_wallet"],
        bump   = airdrop_wallet_state.bump,
        constraint = airdrop_wallet_state.key() == global_state.airdrop_wallet
                     @ NovaPixelError::NotAdmin,
    )]
    pub airdrop_wallet_state: Account<'info, WalletState>,

    /// NVPX token account owned by airdrop_wallet_state PDA.
    /// Jupiter deposits purchased NVPX here.
    #[account(
        mut,
        constraint = airdrop_token_account.owner == airdrop_wallet_state.key()
                     @ NovaPixelError::NotAdmin,
    )]
    pub airdrop_token_account: Account<'info, TokenAccount>,

    /// CHECK: Jupiter v6 program; validated inside jupiter::cpi_swap.
    pub jupiter_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<BuyPackage>,
    package_type: u8,
    jupiter_data: Vec<u8>,   // raw swap instruction bytes from Jupiter API
    sol_amount:   u64,       // exact SOL amount player sends (matches Jupiter quote)
) -> Result<()> {
    require!(ctx.accounts.global_state.is_active(), NovaPixelError::TournamentNotActive);
    require!(!ctx.accounts.global_state.is_paused(), NovaPixelError::ContractPaused);

    let (attempts_to_grant, _nvpx_equiv) = match package_type {
        PACKAGE_STARTER  => (STARTER_ATTEMPTS,  STARTER_NVPX_EQUIV),
        PACKAGE_ADVANCED => (ADVANCED_ATTEMPTS, ADVANCED_NVPX_EQUIV),
        PACKAGE_PRO      => (PRO_ATTEMPTS,       PRO_NVPX_EQUIV),
        _                => return err!(NovaPixelError::InvalidPackageType),
    };

    // ── 1. Transfer SOL from player → sol_vault ───────────────────────────────
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            SolTransfer {
                from: ctx.accounts.player.to_account_info(),
                to:   ctx.accounts.sol_vault.to_account_info(),
            },
        ),
        sol_amount,
    )?;

    // ── 2. Record balance before swap to compute how much NVPX arrived ────────
    let nvpx_before = ctx.accounts.airdrop_token_account.amount;

    // ── 3. Jupiter CPI — sol_vault PDA signs ─────────────────────────────────
    let (_, vault_bump) = Pubkey::find_program_address(&[b"sol_vault"], ctx.program_id);
    let signer_seeds: &[&[&[u8]]] = &[&[b"sol_vault", &[vault_bump]]];

    jupiter::cpi_swap(
        ctx.remaining_accounts,
        jupiter_data,
        signer_seeds,
    )?;

    // ── 4. Verify tokens arrived ───────────────────────────────────────────────
    ctx.accounts.airdrop_token_account.reload()?;
    let nvpx_received = ctx.accounts
        .airdrop_token_account
        .amount
        .checked_sub(nvpx_before)
        .ok_or(NovaPixelError::MathUnderflow)?;

    require!(nvpx_received > 0, NovaPixelError::ZeroTokensReceived);

    // ── 5. Update state ───────────────────────────────────────────────────────
    let pa = &mut ctx.accounts.player_account;
    pa.attempts_balance = pa
        .attempts_balance
        .checked_add(attempts_to_grant)
        .ok_or(NovaPixelError::MathOverflow)?;
    pa.total_sol_spent = pa
        .total_sol_spent
        .checked_add(sol_amount)
        .ok_or(NovaPixelError::MathOverflow)?;

    // Track that these NVPX are now in the airdrop pool earmarked for this pool
    ctx.accounts.airdrop_wallet_state.nvpx_balance = ctx
        .accounts
        .airdrop_wallet_state
        .nvpx_balance
        .checked_add(nvpx_received)
        .ok_or(NovaPixelError::MathOverflow)?;

    emit!(PackagePurchased {
        wallet:       ctx.accounts.player.key(),
        package_type,
        attempts:     attempts_to_grant,
        sol_paid:     sol_amount,
        nvpx_bought:  nvpx_received,
    });

    Ok(())
}
