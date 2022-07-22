use anchor_lang::prelude::*;
pub use solana_program;

mod instructions;
use instructions::{ 
    init_instance::*, 
    init_game::*, 
    register_user::*,
    topup_account::*,
    join_game::*,
};

mod states;
mod processors;
mod errors;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solpoker {
    use instructions::register_user::RegisterUser;

    use super::*;

    pub fn initialize_instance(ctx: Context<InitInstance>, manager_fees_in_bps : u8 ) -> Result<()> {
        processors::process_init_instance::process(ctx, manager_fees_in_bps)
    }

    pub fn initialize_game( ctx: Context<InitGame>, game_id : u32, small_blind : u64, timeout_in_unix_diff: u64) -> Result<()> {
        processors::process_init_game::process(ctx, game_id, small_blind, timeout_in_unix_diff)
    }

    pub fn register_user( ctx: Context<RegisterUser> ) -> Result<()> {
        processors::process_register_user::process(ctx)
    }

    pub fn topup_account( ctx: Context<TopupAccount>, lamports: u64) -> Result<()> {
        processors::process_topup_account::process(ctx, lamports)
    }

}
