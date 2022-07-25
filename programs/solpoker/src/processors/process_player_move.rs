use crate::{*, states::enums::UserState};
use instructions::player_move::PlayerMove;
use states::{enums::UserMoves};
use errors::{ SolPokerErrors };


pub fn process(ctx : Context<PlayerMove>, user_move : UserMoves)-> Result<()> {

    let game = &mut ctx.accounts.game;
    let max_bid_this_round = game.max_bid_this_round;

    let user_option = game.players.iter_mut().find(|x| x.user_pk == ctx.accounts.user.key());
    let user = match user_option {
        Some(x) => x,
        None => return Err(SolPokerErrors::CannotFindUserInGame.into()),
    };

    if user.user_state == UserState::Leaving || user.user_state == UserState::Left {
        return Err(SolPokerErrors::InvalidState.into())
    }

    match user_move {
        UserMoves::Call => {
            if user.current_user_bid >= max_bid_this_round {
                return Ok(())
            }
            let diff = max_bid_this_round - user.current_user_bid;
            user.user_state = UserState::Bid { amount: diff };
        },
        UserMoves::Check => {
            if max_bid_this_round > 0 {
                return Err(SolPokerErrors::CannotCheck.into());
            }
            user.user_state = UserState::Check;
        },
        UserMoves::Fold => {
            if user.user_state == UserState::AllIn {
                return Err(SolPokerErrors::InvalidState.into());
            }
            user.user_state = UserState::Fold;
        },
        UserMoves::Raise {amount} => {
            if amount < max_bid_this_round 
            {
                return Err(SolPokerErrors::RaiseAmountLower.into());
            }
            let diff = amount - user.current_user_bid;
            user.user_state = UserState::Bid { amount: diff };
        }
    }
    Ok(())
}