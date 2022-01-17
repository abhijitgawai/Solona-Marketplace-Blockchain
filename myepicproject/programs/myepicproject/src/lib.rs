//<1>---------------------------------------------------
// use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");      // "program id" and has info for Solana on how to run our program. Anchor has generated this
 
// #[program]
// pub mod myepicproject {
//     use super::*;
//     pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}
//</1>---------------------------------------------------

// //<2>---------------------------------------------------
// use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[program]
// pub mod myepicproject {
//   use super::*;
//   pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
//     Ok(())
//   }
// }

// #[derive(Accounts)]
// pub struct StartStuffOff {}
// //</2>---------------------------------------------------




//<3>---------------------------------------------------
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.                             // &mut to get a "mutable reference" to base_account  // gives us the power to make changes to base_account. Otherwise, we'd simply be working w/ a "local copy" of base_account.
    let base_account = &mut ctx.accounts.base_account;             // we just grab base_account from the StartStuffOff context by doing Context<StartStuffOff>.
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {                                  // here we actually specify how to initialize it and what to hold in our StartStuffOff context
    #[account(init, payer = user, space = 9000)]                   // how we want to initialize BaseAccount.        // if error Transaction simulation failed: Error processing Instruction 0: custom program error: 0x64 change space = 9000 to space = 10000
//                                                                 // init will tell Solana to create a new account owned by our current program
//                                                                 // payer = user tells our program who's paying for the account to be created. In this case, it's the user calling the function.
//                                                                 // allocate 9000 bytes of space for our account
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,                                       // which is data passed into the program that proves to the program that the user calling this program actually owns their wallet account.
    pub system_program: Program <'info, System>,                   // id of 11111111111111111111111111111111
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {                                          // tells our program what kind of account it can make and what to hold inside of it
    pub total_gifs: u64,                                          // Here BaseAccount holds one thing and it's an integer named total_gifs
}

//</3>---------------------------------------------------


