//<1>
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
//</1>

//<2>
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff {}
//</2>


