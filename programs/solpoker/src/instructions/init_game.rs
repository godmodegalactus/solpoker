use anchor_lang::prelude::*;
use anchor_spl::states::Mint;
use crate::states::game;

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(mut)]
    pub admin : Signer<'info>,
}