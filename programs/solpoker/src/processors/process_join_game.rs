use crate::*;
use errors::{SolPokerErrors, check};

pub fn process(ctx : Context<JoinGame>, lamports: u64) -> Result<()> {
    let game = &mut ctx.accounts.game.load_mut()?;
    game.check()?;
    check(game.base_mint == ctx.accounts.user.base_mint.key(), SolPokerErrors::InvalidMint)?;
    let user = &mut ctx.accounts.user;
    game.add_player(ctx.accounts.owner.key(), user, lamports)?;
    Ok(())
}