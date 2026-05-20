use anchor_lang::prelude::*;

declare_id!("EXV3KYGvRm3Xir1gf94hF7LsupEC5scJUUDCpQ3eXnFu");

#[program] // tells anchor these are onchain instructions
pub mod counter_2026_05_19 {
    use super::*; 

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        Ok(()) // returns success
    }


    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // take the current counter value, add 1 safely, then store the result back in the counter
        // .unwrap() - takes the value out of Some(...)
        ctx.accounts.counter.count = ctx.accounts.counter.count.checked_add(1).unwrap();
        Ok(())
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
