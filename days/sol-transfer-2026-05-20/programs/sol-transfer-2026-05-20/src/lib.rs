use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
use solana_program::system_instruction;

declare_id!("GXhPYbWhpv5BVcivPJJ8LETF8DB5xEgUhPGeQ2UtqVad");

#[program]
pub mod sol_transfer_2026_05_20 {
    use super::*;

    pub fn transfer_lamports(ctx: Context<TransferLamports>, amount: u64) -> Result<()> {
        
        // define accounts
        let from_account = &ctx.accounts.from;
        let to_account = &ctx.accounts.to;

        // create transfer ix
        let transfer_instruction = system_instruction::transfer(from_account.key, to_account.key, amount);
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)]
    pub from: Signer<'info>, // signer of the tx, sends SOL
    #[account(mut)]
    pub to: AccountInfo<'info>, // receiver of 'from's SOL
    pub system_program: Program<'info, System>, // system program to handle transfer
}
