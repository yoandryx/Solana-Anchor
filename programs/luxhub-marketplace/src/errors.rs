use anchor_lang::prelude::*;

#[error_code]
pub enum LuxError {
    #[msg("Unauthorized access.")]
    Unauthorized,

    #[msg("Must be called via Squads CPI.")]
    NotCalledBySquads,

    #[msg("Math operation overflowed.")]
    MathOverflow,

    #[msg("Token account mint does not match expected mint.")]
    MintMismatch,

    // Escrow Errors
    #[msg("Escrow has already been completed.")]
    EscrowAlreadyCompleted,

    #[msg("Escrow already has a buyer assigned.")]
    EscrowHasBuyer,

    #[msg("Price cannot be zero.")]
    InvalidPrice,

    #[msg("Only the seller can perform this action.")]
    NotSeller,

    #[msg("Cannot cancel escrow after buyer has deposited funds.")]
    CannotCancelWithBuyer,

    #[msg("Fee cannot exceed 10% (1000 basis points).")]
    FeeTooHigh,

    #[msg("Protocol is currently paused.")]
    ProtocolPaused,

    #[msg("Cannot purchase your own listing.")]
    SelfPurchase,

    #[msg("No buyer assigned to this escrow.")]
    NoBuyer,
}
