use anchor_lang::prelude::*;

declare_id!("EXV3KYGvRm3Xir1gf94hF7LsupEC5scJUUDCpQ3eXnFu");

#[program] // tells anchor these are onchain instructions
pub mod counter_2026_05_19 {
    use super::*; 

    // only instruction
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(()) // returns success
    }
}

// data model for counter
#[account] // store in a solana account
pub struct Counter { 
    pub count: u64, 
}

// defines what accounts the instruction needs
#[derive(Accounts)]
pub struct Initialize {} // empty
