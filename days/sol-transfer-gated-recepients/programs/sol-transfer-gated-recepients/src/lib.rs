use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, pubkey};

declare_id!("3VGupt72thTnFeUgh4WqoA4konjRhcYTyqSqtUzWuDdv");

const ALLOWED_RECEPIENT: Pubkey = pubkey!("9mzJxQzkBcPmx8y1LM4uAh38k21XTGsMhCEj9rn1AqG3");

#[program]
pub mod sol_transfer_gated_recepients {
    use super::*;

    let from = &ctx.accounts.from;
    
    pub fn transfer_lamports(ctx: Context<TransferLamports>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferLamports<'info> { // 'info - assures rust that the borrowed account data lives at least as long as this instruction

    #[account(mut)] // sol balance changing - must be writeable
    pub from: Signer<'info>,

    #[account( // this is where you define ALL restrictions on the account field
        mut,
        address = ALLOWED_RECEPIENT,
    )] // sol balance changing - must be writeable
    pub to: Account<'info>,

    pub system_program: Program<'info, System>,
}
