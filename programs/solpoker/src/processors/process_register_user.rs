use crate::*;
use states::{ meta_data::MetaData, enums::DataType };

use instructions::register_user::RegisterUser;

pub fn process(ctx : Context<RegisterUser>) -> Result<()> {
    ctx.accounts.user.meta_data = MetaData{
        data_type : DataType::User,
        version : 1,
        is_initialized : true,
    };
    ctx.accounts.user.owner_pk = ctx.accounts.owner.key();
    ctx.accounts.user.base_mint = ctx.accounts.base_mint.key();
    ctx.accounts.user.game_context = ctx.accounts.game_context.key();

    ctx.accounts.user.balance_lamports = 0;
    ctx.accounts.user.won_count = 0;
    ctx.accounts.user.lose_count = 0;
    ctx.accounts.user.lamport_won = 0;
    Ok(())
}