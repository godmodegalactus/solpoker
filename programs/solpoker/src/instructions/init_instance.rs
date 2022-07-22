use crate::*;
use crate::states::{ manager::Manager };
use std::{mem::size_of};
use anchor_spl::token::{Mint};

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
        space = 12 + size_of::<Manager>(),
        payer = manager,
    )]
    pub manager_info : Account<'info, Manager>,

    pub system_program : Program<'info, System>,
}