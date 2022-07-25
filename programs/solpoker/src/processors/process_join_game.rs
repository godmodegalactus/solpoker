use crate::*;
use errors::{SolPokerErrors, check};

pub fn process(ctx : Context<JoinGame>, lamports: u64) -> Result<()> {
    ctx.accounts.game.check()?;
    check(ctx.accounts.game.base_mint == ctx.accounts.user.base_mint.key(), SolPokerErrors::InvalidMint)?;
    let user = &mut ctx.accounts.user;
    ctx.accounts.game.add_player(ctx.accounts.owner.key(), user, lamports)?;
    ctx.accounts.game.number_of_users_joined = ctx.accounts.game.number_of_users_joined.saturating_add(1);
    Ok(())
}