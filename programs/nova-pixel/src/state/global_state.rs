use anchor_lang::prelude::*;

// ── Tournament state machine ───────────────────────────────────────────────────
//
//   INACTIVE ──start_tournament()──► ACTIVE ──end_round()──► ROUND_ENDED
//                  ▲                    │                          │
//                  │              end_tournament()           start_tournament()
//                  │                    │                    (next round)
//                  │                    ▼
//                  │                  ENDED ──finalize_airdrop()──► DISTRIBUTION
//                  │                                                      │
//              (can restart)                                      claim_airdrop()
//
//   ACTIVE / ROUND_ENDED ──emergency_pause()──► PAUSED ──unpause()──► ACTIVE
//
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TournamentState {
    Inactive,
    Active,
    RoundEnded,   // between rounds — players may withdraw earnings
    Ended,        // final end — waiting for finalize_airdrop
    Paused,       // emergency pause
    Distribution, // airdrop finalized, players can claim 40% pool
}

impl TournamentState {
    pub const SIZE: usize = 1 + 1; // discriminant + safety pad
}

// ── Main program state PDA ─────────────────────────────────────────────────────
#[account]
pub struct GlobalState {
    // Authority
    pub admin:             Pubkey,      // 32
    pub game_server:       Pubkey,      // 32
    pub multisig_signers:  [Pubkey; 3], // 96
    pub pause_votes:       [bool; 3],   //  3

    // Tournament
    pub tournament_state:      TournamentState, //  2
    pub tournament_start_time: i64,             //  8
    pub tournament_end_time:   i64,             //  8
    pub initialized_at:        i64,             //  8

    // Multi-round tracking
    pub current_round:          u8,   //  1 — 0-indexed round counter
    pub airdrop_finalized:       bool, //  1 — true once finalize_airdrop() is called
    pub authorities_revoked:     bool, //  1 — true after mint+freeze authority permanently revoked

    // Token
    pub nvpx_mint: Pubkey, // 32

    // Aggregate stats (accumulate across ALL rounds)
    pub total_correct_pixels: u64, //  8
    pub total_nvpx_in_game:   u64, //  8
    pub total_nvpx_rewarded:  u64, //  8

    // Wallet PDA pubkeys
    pub liquidity_wallet:    Pubkey, // 32
    pub airdrop_wallet:      Pubkey, // 32
    pub team_wallet:         Pubkey, // 32
    pub development_wallet:  Pubkey, // 32
    pub burn_wallet:         Pubkey, // 32
    pub reserve_wallet:      Pubkey, // 32
    pub buyback_wallet:      Pubkey, // 32

    pub bump: u8, //  1
}

impl GlobalState {
    // 8 discriminator + all fields
    pub const LEN: usize = 8
        + 32 + 32 + (32 * 3) + 3   // admin, game_server, multisig, votes
        + 2 + 8 + 8 + 8             // state, timestamps
        + 1 + 1 + 1                 // current_round, airdrop_finalized, authorities_revoked
        + 32                        // mint
        + 8 + 8 + 8                 // stats
        + (32 * 7)                  // wallet pubkeys
        + 1;                        // bump

    pub fn is_active(&self) -> bool {
        self.tournament_state == TournamentState::Active
    }

    /// True when players can withdraw in-game earnings (between rounds or after end).
    pub fn is_round_ended(&self) -> bool {
        self.tournament_state == TournamentState::RoundEnded
    }

    /// True for any "after active play" state (used by sell_ingame).
    pub fn is_ended(&self) -> bool {
        matches!(
            self.tournament_state,
            TournamentState::Ended | TournamentState::Distribution
        )
    }

    pub fn is_paused(&self) -> bool {
        self.tournament_state == TournamentState::Paused
    }

    /// Count how many multisig signers have voted for pause.
    pub fn pause_vote_count(&self) -> usize {
        self.pause_votes.iter().filter(|&&v| v).count()
    }
}
