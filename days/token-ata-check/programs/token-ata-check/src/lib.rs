// Given the signer's pubkey & token CA, check if signer has an ATA
use anchor_lang::prelude::*;
use anchor_spl::token::{Self, TokenAccount, Token, Mint};

declare_id!("aXr23Zpt1biMAhj5APMECbf4YiQPU3LwjRQuafcYJv8");

#[program]
pub mod token_ata_check {
    use super::*;

    pub fn check_ata(ctx: Context<CheckATA>) -> Result<()> {

        // check that the signer's ATA has a balance above zero
        require!(
            ctx.accounts.signer_ata_amount > 0,
            ErrorCode::NoBalance
        );

        msg!("Signer has valid ATA");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckATA {
    #[derive(Accounts)]
    pub signer: Signer<'info>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Signer does not have valid ATA")],
    NoBalance,
}
