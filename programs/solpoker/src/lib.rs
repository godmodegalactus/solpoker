use anchor_lang::prelude::*;
pub use solana_program;

mod instructions;
use instructions::{ 
    init_instance::*, 
    init_game::*, 
    register_user::*,
    topup_account::*,
    join_game::*,
    leave_game::*,
    update_game::*,
    player_move::*,
};

use states::{ card::Card, enums::{UserMoves} };

mod states;
mod processors;
mod errors;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solpoker {
    use instructions::register_user::RegisterUser;
    use states::enums::UserMoves;

    use super::*;

    pub fn initialize_instance(ctx: Context<InitInstance>, manager_fees_in_bps : u8 ) -> Result<()> {
        processors::process_init_context::process(ctx, manager_fees_in_bps)
    }

    pub fn initialize_game( ctx: Context<InitGame>, small_blind : u64, timeout_in_unix_diff: u64) -> Result<()> {
        processors::process_init_game::process(ctx, small_blind, timeout_in_unix_diff)
    }

    pub fn register_user( ctx: Context<RegisterUser> ) -> Result<()> {
        processors::process_register_user::process(ctx)
    }

    pub fn topup_account( ctx: Context<TopupAccount>, lamports: u64) -> Result<()> {
        processors::process_topup_account::process(ctx, lamports)
    }

    pub fn join_game ( ctx: Context<JoinGame>, lamports: u64) -> Result<()> {
        processors::process_join_game::process(ctx, lamports)
    }

    pub fn leave_game ( ctx: Context<LeaveGame>) -> Result<()> {
        processors::process_leave_game::process(ctx)
    }

    pub fn leave_game_forced ( ctx: Context<LeaveGameForced>) -> Result<()> {
        processors::process_leave_game::process_forced(ctx)
    }

    pub fn update_game (ctx : Context<UpdateGame>, card1 : Card, card2 : Card, card3 : Card) -> Result<()> {
        processors::process_update_game::process(ctx, [card1, card2, card3])
    }

    pub fn player_move (ctx : Context<PlayerMove>, user_move : UserMoves) ->Result<()> {
        processors::process_player_move::process(ctx, user_move)
    }
}
