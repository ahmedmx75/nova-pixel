use anchor_lang::prelude::*;
use crate::constants::MAX_SHIELDS;

// ── Per-player Shield record ────────────────────────────────────────────────
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]
pub struct Shield {
    pub center_x:    u16,  // 2
    pub center_y:    u16,  // 2
    pub size:        u8,   // 1 (3 or 5 — diameter/side length)
    pub expiry_time: i64,  // 8  (unix timestamp; 0 = empty slot)
}

impl Shield {
    pub const SIZE: usize = 2 + 2 + 1 + 8; // 13 bytes

    /// Returns true if this shield slot is occupied and not yet expired.
    pub fn is_active(&self, now: i64) -> bool {
        self.expiry_time > 0 && self.expiry_time > now
    }

    /// Returns true if pixel (x, y) falls within this shield's area.
    pub fn covers(&self, x: u16, y: u16) -> bool {
        let half  = (self.size as i32) / 2;
        let cx    = self.center_x as i32;
        let cy    = self.center_y as i32;
        let tx    = x as i32;
        let ty    = y as i32;
        (cx - half) <= tx && tx <= (cx + half) && (cy - half) <= ty && ty <= (cy + half)
    }
}

// ── Player account PDA ────────────────────────────────────────────────────────
// Seeds: ["player", wallet.key]
#[account]
pub struct PlayerAccount {
    pub wallet_address:       Pubkey,                      // 32
    pub team:                 u8,                          //  1
    pub attempts_balance:     u32,                         //  4
    pub in_game_nvpx_balance: u64,                         //  8
    pub correct_pixels_colored: u64,                       //  8
    pub airdrop_allocation:   u64,                         //  8
    pub airdrop_claimed:      bool,                        //  1
    pub join_timestamp:       i64,                         //  8
    pub total_sol_spent:      u64,                         //  8
    // Fixed-size shield array; expiry_time == 0 means empty slot
    pub active_shields:       [Shield; MAX_SHIELDS],       // 13 * 10 = 130
    pub is_initialized:       bool,                        //  1
    pub bump:                 u8,                          //  1
}

impl PlayerAccount {
    pub const LEN: usize = 8    // discriminator
        + 32 + 1 + 4 + 8 + 8 + 8 + 1 + 8 + 8  // core fields
        + (Shield::SIZE * MAX_SHIELDS)           // shields array
        + 1 + 1;                                 // flags + bump

    // ── Shield helpers ──────────────────────────────────────────────────────

    /// Find the first empty shield slot (expiry == 0 or expired).
    pub fn free_shield_slot(&self, now: i64) -> Option<usize> {
        self.active_shields
            .iter()
            .position(|s| !s.is_active(now))
    }

    /// Check whether this player has an active shield covering (x, y).
    pub fn has_active_shield_at(&self, x: u16, y: u16, now: i64) -> bool {
        self.active_shields
            .iter()
            .any(|s| s.is_active(now) && s.covers(x, y))
    }

    /// Remove the first active shield covering (x, y).  Returns true if one was found.
    pub fn remove_shield_at(&mut self, x: u16, y: u16, now: i64) -> bool {
        for s in self.active_shields.iter_mut() {
            if s.is_active(now) && s.covers(x, y) {
                *s = Shield::default(); // clear slot
                return true;
            }
        }
        false
    }

    /// Count how many shield slots are currently active.
    pub fn active_shield_count(&self, now: i64) -> usize {
        self.active_shields.iter().filter(|s| s.is_active(now)).count()
    }
}
