use crate::*;
use states::{game::Game};

#[derive(Accounts)]
pub struct UpdateGame<'info> {
    #[account()]
    pub oracle : Signer<'info>,

    #[account(mut)]
    pub game : AccountLoader<'info, Game>,
}