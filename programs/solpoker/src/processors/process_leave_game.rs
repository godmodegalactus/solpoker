use crate::{*, states::enums::UserState};
use errors::{SolPokerErrors, check};

pub fn process(ctx : Context<LeaveGame>) -> Result<()> {
    let game = &mut ctx.accounts.game.load_mut()?;
    game.check()?;
    check(game.base_mint == ctx.accounts.user.base_mint.key(), SolPokerErrors::InvalidMint)?;
    game.number_of_users_joined = game.number_of_users_joined.saturating_sub(1);
    //update state for user to leaving
    let find_player = game.find_player_mut(ctx.accounts.owner.key()); 
    match find_player  {
        Some(player) => {
            player.user_state = UserState::Leaving;
            Ok(())
        },
        None => Err(error!(SolPokerErrors::CannotFindUserInGame))
    }
}

pub fn process_forced(ctx: Context<LeaveGameForced>) -> Result<()> {
    let game = &mut ctx.accounts.game.load_mut()?;
    game.check()?;
    check(game.base_mint == ctx.accounts.user.base_mint.key(), SolPokerErrors::InvalidMint)?;
    check(game.game_oracle == ctx.accounts.oracle.key(), SolPokerErrors::InvalidOracle)?;
    game.number_of_users_joined = game.number_of_users_joined.saturating_sub(1);

    let find_player = game.find_player_mut(ctx.accounts.user.owner_pk); 
    match find_player  {
        Some(player) => {
            player.user_state = UserState::Leaving;
            Ok(())
        },
        None => Err(error!(SolPokerErrors::CannotFindUserInGame))
    }
}