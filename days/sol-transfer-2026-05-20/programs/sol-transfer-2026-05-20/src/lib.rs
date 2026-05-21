use anchor_lang::prelude::*;

declare_id!("GXhPYbWhpv5BVcivPJJ8LETF8DB5xEgUhPGeQ2UtqVad");

#[program]
pub mod sol_transfer_2026_05_20 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
