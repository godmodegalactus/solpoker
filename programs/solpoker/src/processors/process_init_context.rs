use crate::*;
use states::meta_data::MetaData;
use states::enums::DataType;

pub fn process( ctx: Context<InitInstance>, manager_fees_in_bps : u8 ) -> Result<()> {
    let game_context = &mut ctx.accounts.game_context;
    game_context.meta_data = MetaData { 
        data_type : DataType::Context, 
        version : 1, 
        is_initialized : true 
    };

    game_context.manager = ctx.accounts.manager.key();
    game_context.base_mint = ctx.accounts.base_mint.key();
    game_context.treasury_account = ctx.accounts.treasury_account.key();
    game_context.manager_fees_in_bps = manager_fees_in_bps;

    game_context.treasury_collected = 0;
    game_context.count_of_games_currently_running = 0;
    Ok(())
}