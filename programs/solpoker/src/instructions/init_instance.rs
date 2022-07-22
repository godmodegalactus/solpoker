use crate::*;
use crate::states::{ game_context::GameContext };
use std::{mem::size_of};
use anchor_spl::token::{ Token, Mint, TokenAccount};

#[derive(Accounts)]
pub struct InitInstance<'info> {
    #[account(mut)]
    pub manager : Signer<'info>,
    #[account()]
    pub base_mint : Box<Account<'info, Mint>>,
    #[account(
        init,
        seeds = [b"solpoker_instance", manager.key().as_ref(), base_mint.key().as_ref()],
        bump,
        space = 12 + size_of::<GameContext>(),
        payer = manager,
    )]
    pub game_context : Box<Account<'info, GameContext>>,

    #[account(
        init,
        seeds = [b"solpoker_manager_treasury", manager.key().as_ref(), base_mint.key().as_ref()],
        bump,
        payer = manager,
        token::mint = base_mint,
        token::authority = manager,
    )]
    pub treasury_account : Box<Account<'info, TokenAccount>>,

    pub token_program : Program<'info, Token>,
    pub system_program : Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}