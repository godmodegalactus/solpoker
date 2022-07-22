use crate::*;
use states::{ game_context::GameContext, user::User, enums::DataType };
use anchor_spl::token::{ TokenAccount, Token };

#[derive(Accounts)]
pub struct TopupAccount<'info> {
    #[account(mut)]
    pub owner : Signer<'info>,

    #[account(mut)]
    pub payer : Signer<'info>,

    #[account(
        constraint = game_context.meta_data.data_type == DataType::Manager,
        constraint = game_context.meta_data.is_initialized == true, 
    )]
    pub game_context : Box<Account<'info, GameContext>>,

    #[account(mut,)]
    pub owner_token_account : Box<Account<'info, TokenAccount>>,

    #[account( mut,
        seeds = [b"solpoker_user", game_context.key().as_ref(), owner.key().as_ref()],
        bump,
        constraint = user.meta_data.data_type == DataType::User,
        constraint = user.meta_data.is_initialized == true,
        constraint = user.game_context == game_context.key(),
    )]
    pub user : Box<Account<'info, User>>,

    #[account( mut,
        constraint = game_context.treasury_account == treasury_account.key(),
        constraint = owner_token_account.mint == treasury_account.mint,
    )]
    pub treasury_account : Box<Account<'info, TokenAccount>>,
    pub token_program : Program<'info, Token>,
}