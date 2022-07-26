use crate::*;
use states::game_context::GameContext;
use states::game::Game;
use std::{mem::size_of};
use anchor_spl::token::{  Mint,};

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(mut)]
    pub manager : Signer<'info>,
    #[account(mut)]
    pub game_oracle : Signer<'info>,
    #[account( mut, constraint = game_context.manager == manager.key() )]
    pub game_context : Box<Account<'info, GameContext>>,
    #[account( constraint = game_context.base_mint == base_mint.key() )]
    pub base_mint : Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [b"solpoker_game", manager.key().as_ref(), base_mint.key().as_ref()],
        bump,
        space = 8 + size_of::<Game>(),
        payer = manager,
    )]
    pub game : Box<Account<'info, Game>>,

    pub system_program : Program<'info, System>,
}