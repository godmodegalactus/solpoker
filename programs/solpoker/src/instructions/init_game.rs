use crate::*;

#[derive(Accounts)]
pub struct InitGame<'info> {
    #[account(mut)]
    pub admin : Signer<'info>,
}