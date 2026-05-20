/// Jupiter Aggregator v6 CPI helper
///
/// Integration pattern:
///   1. Off-chain: call Jupiter's /quote then /swap-instructions API to get raw
///      instruction data and the full account list.
///   2. Pass raw_data + remaining_accounts into buy_package / admin_buyback.
///   3. This module executes the CPI with the on-chain PDA signing.
///
/// The contract never interprets the Jupiter route bytes — it passes them
/// through opaquely, which keeps compatibility with all future Jupiter versions.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
};

pub const JUPITER_V6_PROGRAM_ID: Pubkey =
    anchor_lang::solana_program::pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

/// Validate that the Jupiter program key matches the expected constant.
/// Call this before `cpi_swap` with the key from the account constraint.
pub fn validate_jupiter_program(program_key: &Pubkey) -> Result<()> {
    require_keys_eq!(
        *program_key,
        JUPITER_V6_PROGRAM_ID,
        crate::errors::NovaPixelError::InvalidJupiterProgram
    );
    Ok(())
}

/// Execute a Jupiter swap via CPI.
///
/// * `remaining_accounts` – all accounts Jupiter needs (first account must be
///                          the Jupiter program itself), in the exact order
///                          returned by Jupiter's API
/// * `raw_data`           – raw instruction data bytes from Jupiter's API
/// * `signer_seeds`       – PDA signer seeds (e.g. `sol_vault` or `buyback_wallet`)
pub fn cpi_swap<'info>(
    remaining_accounts: &[AccountInfo<'info>],
    raw_data: Vec<u8>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let metas: Vec<AccountMeta> = remaining_accounts
        .iter()
        .map(|a| AccountMeta {
            pubkey:      *a.key,
            is_signer:   a.is_signer,
            is_writable: a.is_writable,
        })
        .collect();

    invoke_signed(
        &Instruction {
            program_id: JUPITER_V6_PROGRAM_ID,
            accounts:   metas,
            data:        raw_data,
        },
        remaining_accounts,
        signer_seeds,
    )
    .map_err(|_| error!(crate::errors::NovaPixelError::JupiterSwapFailed))
}
