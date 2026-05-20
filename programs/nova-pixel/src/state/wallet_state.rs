use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum WalletType {
    Liquidity,
    Airdrop,
    Team,
    Development,
    Burn,
    Reserve,
    Buyback,
}

/// State PDA for each locked program wallet.
/// The actual tokens live in the ATA owned by this PDA.
#[account]
pub struct WalletState {
    pub wallet_type:        WalletType, //  2
    pub nvpx_balance:       u64,        //  8  — mirrors token account (decremented on spend)
    pub initial_allocation: u64,        //  8
    pub lock_until:         i64,        //  8  — unix ts; 0 = no time lock
    pub is_burned:          bool,       //  1  — used by burn_wallet
    pub bump:               u8,         //  1
}

impl WalletState {
    pub const LEN: usize = 8 + 2 + 8 + 8 + 8 + 1 + 1; // 36 bytes

    pub fn is_time_unlocked(&self, now: i64) -> bool {
        self.lock_until == 0 || now >= self.lock_until
    }
}
