use anchor_lang::prelude::*;

declare_id!("7wEySZY5UT1HNqEFHGve83ufgKsaFhjbVv5yshnYq91b");

#[program] // tells anchor these are onchain instructions
pub mod counter_2026_05_19 {
    use super::*; 

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        // ctx: Context<InitializeCounter>
        // this means - this ix uses the InitializeCounter account layout.
        // anchor should parse the incoming accounts into that struct
        // anchor should validate the constraints before my function body runs
        // InitializeCounter - the checklist. Context<InitializeCounter> - the filled in checklist plus the actual accounts for this call
        ctx.accounts.counter.count = 0;
        // ctx - everything this fn needs to know about this call aka everything Anchor gives us for this ix after matching & validating inputs
        // ctx.accounts - the list of onchain accounts that came with the instruction
        // ctx.accounts.counter - specific account named 'counter' aka the actual counter account this instruction is supposed to work with
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
// <'info> - a rust lifetime used by Anchor for account references
#[derive(Accounts)]
pub struct InitializeCounter<'info> { // creates the checklist for initialize_counter instruction
    // most important line:
    // init - create this account during the ix
    // payer = user - charge the user account to pay for creation
    // space = 8 + 8 - reserve enough bytes for the account data
    #[account(init, payer = user, space = 8 + 8)] // first 8 - Anchor account discriminator, second 8 is our u64 count
    pub counter: Account<'info, Counter>, // store this var based on the Counter shape

    #[account(mut)] // the next accounts can be changed. since lamports will be deducted from the payer, it must be mutable
    pub user: Signer<'info>, // this account must sign the tx. this is the payer for account creation
    pub system_program: Program<'info, System>, // solana uses the system program to create new accounts. init needs this to work.
}

#[derive(Accounts)] // define the required accounts
pub struct Increment<'info> { // checklist for increment instruction

    #[account(mut)] // the counter val will change so this must be mutable
    pub counter: Account<'info, Counter>, // tells Anchor which stored account to load and mutate
}