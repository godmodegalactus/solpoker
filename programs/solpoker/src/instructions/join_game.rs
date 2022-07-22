use crate::*;

use states::{ user::User, game::Game, enums::DataType };

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account()]
    pub owner : Signer<'info>,

    #[account(mut,
        constraint = user.meta_data.data_type == DataType::User,
        constraint = user.meta_data.is_initialized == true,
    )]
    pub user : Account<'info, User>,

    #[account(mut)]
    pub game : AccountLoader<'info, Game>,
}