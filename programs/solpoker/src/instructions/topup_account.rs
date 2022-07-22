use crate::*;
use states::{ manager::Manager, user::User, enums::DataType };
use anchor_spl::token::{ TokenAccount, Token };

#[derive(Accounts)]
pub struct TopupAccount<'info> {
    #[account(mut)]
    pub owner : Signer<'info>,

    #[account(mut)]
    pub payer : Signer<'info>,

    #[account(
        constraint = manager_info.meta_data.data_type == DataType::Manager,
        constraint = manager_info.meta_data.is_initialized == true, 
    )]
    pub manager_info : Box<Account<'info, Manager>>,

    #[account(mut,)]
    pub owner_token_account : Box<Account<'info, TokenAccount>>,

    #[account( mut,
        seeds = [b"solpoker_user", owner_token_account.mint.key().as_ref(), owner.key().as_ref()],
        bump,
        constraint = user.meta_data.data_type == DataType::User,
        constraint = user.meta_data.is_initialized == true,
    )]
    pub user : Box<Account<'info, User>>,

    #[account( mut,
        seeds = [
            b"solpoker_manager_treasury", 
            manager_info.manager_pk.as_ref(), 
            owner_token_account.mint.key().as_ref()],
        bump,
        constraint = owner_token_account.mint == treasury_account.mint,
    )]
    pub treasury_account : Box<Account<'info, TokenAccount>>,
    pub token_program : Program<'info, Token>,
}