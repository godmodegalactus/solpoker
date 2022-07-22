use crate::*;
use states::manager::Manager;
use states::game::Game;
use std::{mem::size_of};
use anchor_spl::token::{  Mint,};

#[derive(Accounts)]
#[instruction(game_id: u8)]
pub struct InitGame<'info> {
    #[account(mut)]
    pub manager : Signer<'info>,
    #[account(mut)]
    pub game_oracle : Signer<'info>,
    #[account( constraint = manager_info.manager_pk == manager.key() )]
    pub manager_info : Box<Account<'info, Manager>>,
    #[account( constraint = manager_info.base_mint == base_mint.key() )]
    pub base_mint : Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [b"solpoker_game", manager.key().as_ref(), base_mint.key().as_ref(), &[game_id]],
        bump,
        space = 12 + size_of::<Game>(),
        payer = manager,
    )]
    pub game : AccountLoader<'info, Game>,

    pub system_program : Program<'info, System>,
}