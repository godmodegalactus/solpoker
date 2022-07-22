use crate::*;

use states::{ game_context::GameContext, user::User, enums::DataType };
use anchor_spl::token::{  Mint };
use std::{mem::size_of};

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(mut)]
    pub owner : Signer<'info>,

    #[account(
        constraint = game_context.meta_data.data_type == DataType::Manager,
        constraint = game_context.meta_data.is_initialized == true, 
    )]
    pub game_context : Box<Account<'info, GameContext>>,

    #[account( constraint = game_context.base_mint == base_mint.key() )]
    pub base_mint : Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [b"solpoker_user", game_context.key().as_ref(), owner.key().as_ref()],
        bump,
        space = 12 + size_of::<User>(),
        payer = owner,
    )]
    pub user : Box<Account<'info, User>>,

    pub system_program : Program<'info, System>,
}