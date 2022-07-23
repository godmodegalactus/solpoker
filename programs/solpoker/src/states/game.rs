use crate::*;
use crate::states::{ 
    meta_data::MetaData, 
    card::Card, 
    enums::{DataType, CurrentGameState, UserState, MAX_NUMBER_OF_PLAYERS},
    user_data::UserData,
    user::User,
 };

 use errors::{SolPokerErrors, check};

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
    pub big_blind_user_index : u8,
    // number of user joined
    pub number_of_users_joined : u8,
    // pots
    pub current_pot : u64,
    // bid start index
    pub bid_start_index : u8,
    // bids this round
    pub bids_this_round : u64,
    // max bid by a player this round
    pub max_bid_this_round : u64,

    // will be opened as game progresses
    pub card1 : Card,
    pub card2 : Card,
    pub card3 : Card,
    pub card4 : Card,
    pub card5 : Card,
    // current players playing
    pub current_players : [UserData; 10],
    pub current_player : u8,
    // when player updates the game
    pub can_update : bool,
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
            card1 : Card::default(),
            card2 : Card::default(),
            card3 : Card::default(),
            card4 : Card::default(),
            card5 : Card::default(),
            current_players : [UserData::default(); 10],
            big_blind_user_index : 0,
            number_of_users_joined : 0,
            max_bid_this_round : 0,
            current_pot : 0,
            bids_this_round : 0,
            bid_start_index : 0,
            current_player : 0,
            can_update :false,
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

        let max_number_of_players:usize = self.max_number_of_players as usize;
        for i in 0..max_number_of_players {
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
        for i in 0..self.max_number_of_players as usize {
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

    pub fn find_player_mut(&mut self, user_pk : Pubkey) -> Option<&mut UserData> {
        for i in 0..self.max_number_of_players as usize {
            if self.current_players[i].user_pk == user_pk {
                return Some(&mut self.current_players[i]);
            }
        }
        None
    }
    
    pub fn find_player(self, user_pk : Pubkey) -> Option<UserData> {
        for i in 0..self.max_number_of_players as usize {
            if self.current_players[i].user_pk == user_pk {
                return Some(self.current_players[i]);
            }
        }
        None
    }

    pub fn get_next_player_index(self, index: usize) -> Option<usize> {
        let max_number_of_players : usize = self.max_number_of_players as usize;
        let mut current_index = if index == max_number_of_players { 0 } else {index + 1};
        while current_index != index {
            let player = self.current_players[current_index];
            let ignore_in_states = vec![UserState::Fold, UserState::AllIn, UserState::Leaving];

            if player.user_pk != Pubkey::default() && !ignore_in_states.iter().any(|x| *x == player.user_state) {
                return Some(current_index);
            } 
            current_index = if current_index == max_number_of_players { 0 } else {current_index + 1};    
        }
        None
    }

    pub fn transfer_from_player_at_index(&mut self, player_index: usize, lamports : u64) -> Result<()> {
        let current_player = &mut self.current_players[player_index];

        if current_player.user_state == UserState::AllIn {
            return Ok(());
        }
        
        // if player has enough balance
        if current_player.user_balance > lamports {
            current_player.user_balance = current_player.user_balance.saturating_sub(lamports);
            self.current_pot = self.current_pot.saturating_add(lamports);
            self.bids_this_round = self.bids_this_round.saturating_add(lamports);
            current_player.user_stakes = current_player.user_stakes.saturating_add(lamports);
            current_player.current_user_bid = current_player.current_user_bid.saturating_add(lamports);
        }
        // if player has less balance then required we create a new pot / player goes allin
        else {
            self.current_pot = self.current_pot.saturating_add(current_player.user_balance);
            self.bids_this_round = self.bids_this_round.saturating_add(lamports);
            current_player.user_stakes = current_player.user_stakes.saturating_add(current_player.user_balance);
            current_player.current_user_bid = current_player.current_user_bid.saturating_add(current_player.user_balance);
            current_player.user_balance = 0;
            current_player.user_state = UserState::AllIn;
        }
        Ok(())
    }

    pub fn reset_round(&mut self){
        self.bids_this_round = 0;
        self.max_bid_this_round = 0;

        for i in 0..self.max_number_of_players as usize {
            if self.current_players[i].user_pk != Pubkey::default() {
                self.current_players[i].current_user_bid = 0;
            }
        }
    }
}