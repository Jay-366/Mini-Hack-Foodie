use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("YourProgramIDGoesHere");

#[program]
pub mod food_order_system {
    use super::*;

    pub fn process_payment(ctx: Context<ProcessPayment>, amount: u64) -> Result<()> {
        let payment_data = &mut ctx.accounts.payment_data;

        // Record the amount paid and the buyer's public key
        payment_data.buyer = *ctx.accounts.user.key;
        payment_data.amount = amount;

        // Transfer SOL from the user to the merchant
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            ctx.accounts.user.key,
            ctx.accounts.merchant.key,
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.merchant.to_account_info(),
            ],
        )?;

        // Log successful payment
        msg!("Payment of {} lamports processed for user: {}", amount, ctx.accounts.user.key);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProcessPayment<'info> {
    /// The account paying for the order
    #[account(mut)]
    pub user: Signer<'info>,

    /// The merchant receiving payment
    #[account(mut)]
    pub merchant: SystemAccount<'info>,

    /// Data account to log payment information
    #[account(init_if_needed, payer = user, space = 8 + 32 + 8)]
    pub payment_data: Account<'info, PaymentData>,

    /// System program for handling transfers
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PaymentData {
    pub buyer: Pubkey, // Buyer's public key
    pub amount: u64,   // Amount paid in lamports
}
