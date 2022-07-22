use crate::*;

use states::{ manager::Manager, user::User, enums::DataType };
use anchor_spl::token::{  Mint };
use std::{mem::size_of};

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(mut)]
    pub owner : Signer<'info>,

    #[account(
        constraint = manager_info.meta_data.data_type == DataType::Manager,
        constraint = manager_info.meta_data.is_initialized == true, 
    )]
    pub manager_info : Box<Account<'info, Manager>>,

    #[account( constraint = manager_info.base_mint == base_mint.key() )]
    pub base_mint : Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [b"solpoker_user", base_mint.key().as_ref(), owner.key().as_ref()],
        bump,
        space = 12 + size_of::<User>(),
        payer = owner,
    )]
    pub user : Box<Account<'info, User>>,

    pub system_program : Program<'info, System>,
}