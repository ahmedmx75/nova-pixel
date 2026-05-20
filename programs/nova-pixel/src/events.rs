use anchor_lang::prelude::*;

#[event]
pub struct PlayerConnected {
    pub wallet:    Pubkey,
    pub team:      u8,
    pub timestamp: i64,
}

#[event]
pub struct PackagePurchased {
    pub wallet:       Pubkey,
    pub package_type: u8,
    pub attempts:     u32,
    pub sol_paid:     u64,
    pub nvpx_bought:  u64,
}

#[event]
pub struct PixelColored {
    pub wallet:     Pubkey,
    pub team:       u8,
    pub x:          u16,
    pub y:          u16,
    pub is_correct: bool,
    pub nvpx_reward: u64,
}

#[event]
pub struct PixelCaptured {
    pub attacker_wallet:  Pubkey,
    pub defender_wallet:  Pubkey,
    pub x:                u16,
    pub y:                u16,
    pub nvpx_transferred: u64,
}

#[event]
pub struct ItemPurchased {
    pub wallet:    Pubkey,
    pub item_type: u8,
    pub sol_cost:  u64,
}

#[event]
pub struct ShieldActivated {
    pub wallet:   Pubkey,
    pub team:     u8,
    pub center_x: u16,
    pub center_y: u16,
    pub size:     u8,
    pub expiry:   i64,
}

#[event]
pub struct RocketFired {
    pub wallet:          Pubkey,
    pub target_x:        u16,
    pub target_y:        u16,
    pub shield_destroyed: bool,
}

#[event]
pub struct InGameSell {
    pub wallet:          Pubkey,
    pub nvpx_sold:       u64,
    pub penalty_amount:  u64,
    pub tax_amount:      u64,
    pub received_amount: u64,
}

#[event]
pub struct RoundEnded {
    pub round:                u8,
    pub timestamp:            i64,
    pub total_correct_pixels: u64,
}

#[event]
pub struct TournamentStarted {
    pub timestamp: i64,
}

#[event]
pub struct TournamentEnded {
    pub timestamp:            i64,
    pub total_correct_pixels: u64,
    pub total_nvpx_in_game:   u64,
}

#[event]
pub struct EmergencyPaused {
    pub reason:    String,
    pub signer:    Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct EmergencyUnpaused {
    pub signer:    Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AirdropClaimed {
    pub wallet:    Pubkey,
    pub amount:    u64,
    pub timestamp: i64,
}

#[event]
pub struct TokensBurned {
    pub amount:    u64,
    pub timestamp: i64,
}

#[event]
pub struct BuybackExecuted {
    pub sol_spent:   u64,
    pub nvpx_bought: u64,
    pub timestamp:   i64,
}

#[event]
pub struct MintAuthorityRevoked {
    pub mint:      Pubkey,
    pub timestamp: i64,
}
