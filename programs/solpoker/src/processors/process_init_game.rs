use crate::{*, states::enums::CurrentGameState};
use states::{ meta_data::MetaData };
use instructions::init_game::InitGame;


pub fn process( ctx: Context<InitGame>, game_id : u32, small_blind : u64, timeout_in_unix_diff: u64 ) -> Result<()> {
    let game = &mut ctx.accounts.game.load_init()?;

    game.meta_data = MetaData {
        data_type : states::enums::DataType::Game,
        version : 1,
        is_initialized : true,
    };
    game.game_id = game_id;
    game.game_oracle = ctx.accounts.game_oracle.key();
    game.base_mint = ctx.accounts.game_oracle.key();
    game.game_number = 0;
    game.small_blind = small_blind;

    let clock = solana_program::clock::Clock::get()?;
    game.timeout_in_unix_diff = timeout_in_unix_diff;
    game.last_update_time = clock.unix_timestamp as u64;
    game.current_state = CurrentGameState::NotYetStarted;
    ctx.accounts.game_context.count_of_games_currently_running = ctx.accounts.game_context.count_of_games_currently_running.saturating_add(1);

    Ok(())
}