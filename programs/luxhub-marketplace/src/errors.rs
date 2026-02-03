use anchor_lang::prelude::*;

#[error_code]
pub enum LuxError {
    #[msg("Unauthorized access.")]
    Unauthorized,

    #[msg("Must be called via Squads CPI.")]
    NotCalledBySquads,

    #[msg("Math operation overflowed.")]
    MathOverflow,

    // NFT Management Errors
    #[msg("Caller is not the mint authority.")]
    NotMintAuthority,

    #[msg("Caller is not the freeze authority.")]
    NotFreezeAuthority,

    #[msg("Invalid token amount. Expected exactly 1 for NFT.")]
    InvalidTokenAmount,

    #[msg("Token account mint does not match expected mint.")]
    MintMismatch,

    #[msg("Caller is not a delegate for this token account.")]
    NotDelegate,

    #[msg("NFT is already frozen.")]
    AlreadyFrozen,

    #[msg("NFT is not frozen.")]
    NotFrozen,

    #[msg("NFT has already been burned.")]
    AlreadyBurned,

    #[msg("Invalid recipient address.")]
    InvalidRecipient,

    #[msg("Token account is empty.")]
    EmptyTokenAccount,

    // Escrow Update Errors
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
}
