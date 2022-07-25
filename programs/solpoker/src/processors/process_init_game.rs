use crate::{*, states::enums::CurrentGameState};
use states::{ meta_data::MetaData, enums::MAX_NUMBER_OF_PLAYERS };
use instructions::init_game::InitGame;


pub fn process( ctx: Context<InitGame>, small_blind : u64, timeout_in_unix_diff: u64 ) -> Result<()> {
    let game = &mut ctx.accounts.game;

    game.meta_data = MetaData {
        data_type : states::enums::DataType::Game,
        version : 1,
        is_initialized : true,
    };
    game.manager = ctx.accounts.manager.key();
    game.game_context = ctx.accounts.game_context.key();
    game.game_oracle = ctx.accounts.game_oracle.key();
    game.base_mint = ctx.accounts.base_mint.key();
    
    game.game_number = 0;
    game.current_pot = 0;
    game.small_blind = small_blind;
    game.max_number_of_players = MAX_NUMBER_OF_PLAYERS;

    let clock = solana_program::clock::Clock::get()?;
    game.timeout_in_unix_diff = timeout_in_unix_diff;
    game.last_update_time = clock.unix_timestamp as u64;
    game.current_state = CurrentGameState::NotYetStarted;
    ctx.accounts.game_context.count_of_games_currently_running = ctx.accounts.game_context.count_of_games_currently_running.saturating_add(1);

    Ok(())
}