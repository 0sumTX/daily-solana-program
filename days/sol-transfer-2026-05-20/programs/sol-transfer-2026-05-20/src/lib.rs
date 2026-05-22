use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("2u184YBfz87BJQ6AXYXYWe6ocez2xFvQLaSFNmxhTUrQ");

#[program]
pub mod sol_transfer_2026_05_20 {
    use super::*;

    pub fn transfer_lamports(ctx: Context<TransferLamports>, amount: u64) -> Result<()> {
        
        // define accounts
        let from_account = &ctx.accounts.from;
        let to_account = &ctx.accounts.to;

        // create transfer ix based on the system program
        let transfer_instruction = system_instruction::transfer(from_account.key, to_account.key, amount);
        
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                from_account.to_account_info(),
                to_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)] // account must be writeable (changing SOL balance)
    pub from: Signer<'info>, // signer of the tx, sends SOL
    #[account(mut)] // account must be writeable (changing SOL balance)
    pub to: UncheckedAccount<'info>, // receiver of 'from's SOL
    pub system_program: Program<'info, System>, // system program to handle transfer
}
