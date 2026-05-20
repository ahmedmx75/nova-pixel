use anchor_lang::prelude::*;

#[error_code]
pub enum NovaPixelError {
    // ── Tournament state ──────────────────────────────────────────────────────
    #[msg("Tournament is not active")]
    TournamentNotActive,
    #[msg("Tournament is already active")]
    TournamentAlreadyActive,
    #[msg("Tournament is already ended")]
    TournamentAlreadyEnded,
    #[msg("Tournament has not ended yet")]
    TournamentNotEnded,
    #[msg("Contract is paused — emergency only")]
    ContractPaused,
    #[msg("Contract is not paused")]
    NotPaused,

    // ── Authorization ─────────────────────────────────────────────────────────
    #[msg("Not authorized: admin only")]
    NotAdmin,
    #[msg("Not authorized: game server only")]
    NotGameServer,
    #[msg("Not a registered multisig signer")]
    NotMultisigSigner,
    #[msg("Signer already voted for pause")]
    AlreadyVoted,
    #[msg("Insufficient pause votes (need 2-of-3)")]
    InsufficientPauseVotes,

    // ── Player / account ──────────────────────────────────────────────────────
    #[msg("Player account not initialized — call connect_player first")]
    PlayerNotInitialized,
    #[msg("Invalid team: must be 0, 1, or 2")]
    InvalidTeam,
    #[msg("Insufficient attempts balance")]
    InsufficientAttempts,
    #[msg("Insufficient in-game NVPX balance")]
    InsufficientBalance,

    // ── Packages & items ──────────────────────────────────────────────────────
    #[msg("Invalid package type")]
    InvalidPackageType,
    #[msg("Invalid item type")]
    InvalidItemType,
    #[msg("Insufficient SOL sent for this item")]
    InsufficientSolForItem,

    // ── Pixel mechanics ───────────────────────────────────────────────────────
    #[msg("Pixel coordinates out of canvas bounds")]
    InvalidPixelCoords,
    #[msg("Target pixel is protected by a shield")]
    PixelShielded,
    #[msg("No active shield found at target location")]
    NoShieldAtTarget,
    #[msg("Player has reached the maximum number of active shields")]
    TooManyShields,
    #[msg("Zero pixel value — must be positive")]
    ZeroPixelValue,

    // ── Airdrop ───────────────────────────────────────────────────────────────
    #[msg("Airdrop already claimed")]
    AirdropAlreadyClaimed,
    #[msg("No airdrop allocation for this player")]
    NoAirdropAllocation,
    #[msg("Airdrop pool has insufficient funds")]
    AirdropPoolInsufficient,

    // ── Wallet locks ──────────────────────────────────────────────────────────
    #[msg("Wallet is still time-locked")]
    WalletLocked,
    #[msg("Tokens have already been burned")]
    AlreadyBurned,
    #[msg("Mint and freeze authorities have already been permanently revoked")]
    AuthorityAlreadyRevoked,

    // ── Math ──────────────────────────────────────────────────────────────────
    #[msg("Arithmetic overflow")]
    MathOverflow,
    #[msg("Arithmetic underflow")]
    MathUnderflow,
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("Invalid sell amount — must be positive")]
    InvalidSellAmount,

    // ── Jupiter ───────────────────────────────────────────────────────────────
    #[msg("Jupiter swap CPI failed")]
    JupiterSwapFailed,
    #[msg("Invalid Jupiter program ID")]
    InvalidJupiterProgram,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Jupiter returned zero tokens")]
    ZeroTokensReceived,
}
