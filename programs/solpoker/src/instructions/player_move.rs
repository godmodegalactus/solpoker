use crate::*;
use states::{game::Game, enums::UserMoves};

#[derive(Accounts)]
#[instruction(user_move : UserMoves)]
pub struct PlayerMove<'info> {
    #[account()]
    pub user : Signer<'info>,

    #[account(mut)]
    pub game : Box<Account<'info, Game>>,
}