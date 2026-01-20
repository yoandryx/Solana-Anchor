// instructions/update_price.rs
use anchor_lang::prelude::*;
use crate::UpdatePrice;
use crate::errors::LuxError;

/// Handler for updating the sale price of an escrow listing.
///
/// # Arguments
/// * `ctx` - The update price context
/// * `new_price` - The new sale price in lamports (must be > 0)
///
/// # Returns
/// * `Result<()>` - Ok if successful
///
/// # Security
/// - Only the original seller (initializer) can update the price
/// - Cannot update price after escrow is completed
/// - Cannot update price after a buyer is assigned
pub fn handler(ctx: Context<UpdatePrice>, new_price: u64) -> Result<()> {
    // Validate new price
    require!(new_price > 0, LuxError::InvalidPrice);

    let escrow = &mut ctx.accounts.escrow;

    // Log the price update for transparency
    msg!(
        "Updating escrow price: {} -> {} (seller: {})",
        escrow.sale_price,
        new_price,
        ctx.accounts.seller.key()
    );

    // Update the sale price
    escrow.sale_price = new_price;

    // Also update taker_amount if it was set to the sale price
    // This ensures consistency between the two price fields
    if escrow.taker_amount == escrow.sale_price {
        escrow.taker_amount = new_price;
    }

    msg!("Escrow price updated successfully to {} lamports", new_price);

    Ok(())
}
