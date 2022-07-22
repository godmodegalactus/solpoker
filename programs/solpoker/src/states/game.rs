use crate::*;
use crate::states::{ 
    meta_data::MetaData, 
    card::Card, 
    enums::{CurrentGameState, MAX_NUMBER_OF_PLAYERS},
    user_data::UserData,
    user::User,
 };

 use errors::{SolPokerErrors, check};

use super::enums::DataType;

#[account(zero_copy)]
pub struct Game {
    pub meta_data : MetaData,
    // game id
    pub game_id : u32,
    // oracle
    pub game_oracle : Pubkey,
    // base mint
    pub base_mint : Pubkey,
    // number which always increases
    pub game_number : u32,
    // small blind in lamports
    pub small_blind : u64,
    // max number of player
    pub max_number_of_players : u8,
    // timeouts in unix time
    pub timeout_in_unix_diff : u64,
    // last updated time
    pub last_update_time : u64,
    // current state 
    pub current_state: CurrentGameState,
    // small blind user index
    pub small_blind_user_index : u8,
    // number of user joined
    pub number_of_users_joined : u8,
    // pots / as pots can be splitted on allins
    pots: [u64; 10],
    current_pot_index : u8,
    // will be opened as game progresses
    pub card1 : Card,
    pub card2 : Card,
    pub card3 : Card,
    pub card4 : Card,
    pub card5 : Card,
    // current players playing
    pub current_players : [UserData; 10],
}

impl Default for Game {
    fn default() -> Self {
        Game {
            meta_data : MetaData { data_type: states::enums::DataType::Unknown, version: 0, is_initialized: false },
            game_id : 0,
            game_oracle : Pubkey::default(),
            base_mint : Pubkey::default(),
            game_number : 0,
            small_blind : 0,
            max_number_of_players : MAX_NUMBER_OF_PLAYERS,
            timeout_in_unix_diff : 0,
            last_update_time : 0,
            current_state : CurrentGameState::NotYetStarted,
            pots : [0; 10],
            current_pot_index : 0,
            card1 : Card::default(),
            card2 : Card::default(),
            card3 : Card::default(),
            card4 : Card::default(),
            card5 : Card::default(),
            current_players : [UserData::default(); 10],
            small_blind_user_index : 0,
            number_of_users_joined : 0,
        }
    }
}

impl Game {

    pub fn check(self) -> Result<()> {
        if self.meta_data.data_type == DataType::Game && self.meta_data.is_initialized == true {
            Ok(())
        }
        else {
            Err(error!(SolPokerErrors::GameNotCorrectlyInitialized))
        }
    }

    pub fn add_player(&mut self, user_pk : Pubkey, user : &mut Account<User>, transfer_lamports : u64) -> Result<()> {
        let funds_to_transfer = transfer_lamports.min(user.balance_lamports);
        check(funds_to_transfer > self.small_blind, SolPokerErrors::AmountLessThanSmallBlind)?;

        let player = UserData::new(user_pk, user.key(), funds_to_transfer);

        for i in 0..10 {
            if self.current_players[i].user_pk == Pubkey::default() {
                // remove staked money from user balance
                user.balance_lamports = user.balance_lamports.saturating_sub(funds_to_transfer);
                self.current_players[i] = player;
                self.number_of_users_joined = self.number_of_users_joined.saturating_add(1);
                return Ok(());
            }
        }
        Err(error!(errors::SolPokerErrors::GameIsFull))
    }

    pub fn remove_player(&mut self, user_pk : Pubkey, user : &mut Account<User>,) -> Result<()> {
        for i in 0..10 {
            if self.current_players[i].user_pk == user_pk {
                // return money staked in game
                user.balance_lamports = user.balance_lamports.checked_add(self.current_players[i].user_balance).unwrap();
                self.current_players[i] = UserData::default();
                self.number_of_users_joined = self.number_of_users_joined.saturating_sub(1);
                return Ok(());
            }
        }
        Err(error!(SolPokerErrors::CannotFindUserInGame))
    }
}