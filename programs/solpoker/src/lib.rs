use anchor_lang::prelude::*;
pub use solana_program;
use std::{mem::size_of};

mod instructions;
use instructions::init_instance::*;

mod states;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solpoker {
    use super::*;

    pub fn initialize(_ctx: Context<InitInstance>) -> Result<()> {
        Ok(())
    }
}
