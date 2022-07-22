use crate::*;

use states::{ user::User, game::Game, enums::DataType };

#[derive(Accounts)]
pub struct LeaveGame<'info> {
    #[account()]
    pub owner : Signer<'info>,

    #[account(mut,
        constraint = user.meta_data.data_type == DataType::User,
        constraint = user.meta_data.is_initialized == true,
        constraint = user.owner_pk == owner.key(),
    )]
    pub user : Account<'info, User>,

    #[account(mut)]
    pub game : AccountLoader<'info, Game>,
}

#[derive(Accounts)]
pub struct LeaveGameForced<'info> {
    #[account()]
    pub oracle : Signer<'info>,

    #[account(mut,
        constraint = user.meta_data.data_type == DataType::User,
        constraint = user.meta_data.is_initialized == true,
    )]
    pub user : Account<'info, User>,

    #[account(mut)]
    pub game : AccountLoader<'info, Game>,
}