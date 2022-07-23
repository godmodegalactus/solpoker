use crate::{*, states::{card::Card}};
use instructions::update_game::UpdateGame;
use states::{ enums::CurrentGameState, enums::UserState };
use errors::{SolPokerErrors};


pub fn process(ctx : Context<UpdateGame>, cards : [Card; 3]) -> Result<()> {
    let clock = solana_program::clock::Clock::get()?;

    let game = &mut ctx.accounts.game.load_mut()?;
    if game.can_update || game.last_update_time + game.timeout_in_unix_diff < clock.unix_timestamp as u64 {
        Err(error!(SolPokerErrors::LastStateNotInTimeout))
    }
    else {
        game.last_update_time = clock.unix_timestamp as u64;
        match game.current_state {
            //----------------------------------------------------
            CurrentGameState::NotYetStarted => {
                // check how many player are still playing
                let number_of_players = game.current_players.iter().filter(|x| x.user_pk != Pubkey::default()).count();
                if number_of_players > 2 {
                    // update states for the players
                    for player_index in 0..number_of_players {
                        game.current_players[player_index].user_state = UserState::WaitingForCards;
                    }
                    
                    // update small blind player index and transfer small and big blind
                    let next_player_index = game.get_next_player_index(game.big_blind_user_index as usize).unwrap();
                    game.big_blind_user_index = next_player_index as u8;
                    game.bid_start_index = next_player_index as u8;

                    let big_blind_player_index = next_player_index as usize;
                    let small_blind = game.small_blind;
                    let small_blind_player_index = game.get_next_player_index(big_blind_player_index).unwrap();
                    game.transfer_from_player_at_index( big_blind_player_index, small_blind.saturating_mul(2))?;
                    game.transfer_from_player_at_index( small_blind_player_index, small_blind)?;

                    game.current_player = game.get_next_player_index(small_blind_player_index).unwrap() as u8;
                    // update state
                    game.current_state = CurrentGameState::NoCardsShown;
                    Ok(())
                }
                else {
                    Err(SolPokerErrors::NotEnoughPlayers.into())    
                }
            },
            CurrentGameState::NoCardsShown | CurrentGameState::ThreeCardsShown | CurrentGameState::FourCardsShown | CurrentGameState::AllCardsShown => {
                // check response from current player
                let player_index = game.current_player as usize;
                match game.current_players[player_index].user_state {
                    // check of fold in invalid cases
                    UserState::Check | UserState::WaitingForCards | UserState::WaitingForResponse | UserState::WaitingForTurn | UserState::WaitingToStart => {
                        if game.bids_this_round > 0 {
                            game.current_players[player_index].user_state = UserState::Fold;
                        }
                        else {
                            game.current_players[player_index].user_state = UserState::WaitingForTurn;
                        }
                    },
                    UserState::AllIn | UserState::Leaving | UserState::Fold => {
                        // do nohing
                    },
                    UserState::Bid { amount } => {
                        let total_player_bid = game.current_players[player_index].current_user_bid;
                        if total_player_bid >= game.max_bid_this_round {
                            game.transfer_from_player_at_index( player_index, amount)?;
                            game.current_players[player_index].user_state = UserState::WaitingForTurn;
                            if total_player_bid > game.max_bid_this_round {
                                game.max_bid_this_round = total_player_bid;
                                game.bid_start_index = game.current_player;
                            }
                        }
                    }
                    // _ => {
                    //     // for no reponse from the player / folding
                    //     player.user_state = UserState::Fold;
                    // }
                };

                game.current_player =  game.get_next_player_index(player_index).unwrap() as u8;
                // when the whole circle is finished we move to the next step
                if game.current_player == game.bid_start_index {
                    game.reset_round();
                    match game.current_state {
                        CurrentGameState::NoCardsShown => {
                            game.current_state = CurrentGameState::ThreeCardsShown;
                            if cards.iter().all(|x| x.valid()) {
                                game.card1 = cards[0];
                                game.card2 = cards[1];
                                game.card3 = cards[2];
                            } else {
                                return Err(SolPokerErrors::NotEnoughCards.into());
                            }
                        },
                        CurrentGameState::ThreeCardsShown => {
                            game.current_state = CurrentGameState::FourCardsShown;
                            if cards[0].valid() {
                                game.card4 = cards[0];
                            } else {
                                return Err(SolPokerErrors::NotEnoughCards.into());
                            }
                        },
                        CurrentGameState::FourCardsShown => {
                            game.current_state = CurrentGameState::AllCardsShown;
                            if cards[0].valid() {
                                game.card5 = cards[0];
                            } else {
                                return Err(SolPokerErrors::NotEnoughCards.into());
                            }
                        },
                        CurrentGameState::AllCardsShown => {
                            game.current_state = CurrentGameState::GameEnded;
                        },
                        _=> { 
                            return Err(SolPokerErrors::InvalidState.into());
                        }
                    }
                }
                
                Ok(())
            }
            _ => {
                Err(error!(SolPokerErrors::UnknownState))
            }
        }
    }

}