use crate::*;
use errors::{SolPokerErrors, check};

pub fn process(ctx : Context<LeaveGame>) -> Result<()> {
    let game = &mut ctx.accounts.game.load_mut()?;
    game.check()?;
    check(game.base_mint == ctx.accounts.user.base_mint.key(), SolPokerErrors::InvalidMint)?;
    let user = &mut ctx.accounts.user;
    game.remove_player(ctx.accounts.owner.key(), user)?;
    Ok(())
}

pub fn process_forced(ctx: Context<LeaveGameForced>) -> Result<()> {
    let game = &mut ctx.accounts.game.load_mut()?;
    game.check()?;
    check(game.base_mint == ctx.accounts.user.base_mint.key(), SolPokerErrors::InvalidMint)?;
    check(game.game_oracle == ctx.accounts.oracle.key(), SolPokerErrors::InvalidOracle)?;
    let user = &mut ctx.accounts.user;
    game.remove_player(user.owner_pk, user)?;
    Ok(())
}