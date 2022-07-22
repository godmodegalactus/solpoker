use crate::*;
use states::card::Card;

use super::user::User;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
#[repr(C)]
/// Stores meta information about the `Account` on chain
pub struct UserState {
    // user pk
    pub user_pk : Pubkey,
    // user data / address of User struct for the given user
    pub user_data : Pubkey,
    // user balance in lamports for the game / user
    pub user_balance : u64,
    // user stakes in current game
    pub user_stakes : u64,
    // player pot index / used when player allins
    pub pot_index : u8,
    // updated when game ends to calculate the winner
    pub card_1 : Card,
    pub card_2 : Card,

    // stat data
    pub lamports_won : u64,
}

impl Default for UserState {

    fn default() -> Self {
        UserState {
            user_pk: Pubkey::default(),
            user_data: Pubkey::default(),
            user_balance: 0,
            user_stakes: 0,
            pot_index : 0,
            card_1 : Card::default(),
            card_2 : Card::default(),
            lamports_won : 0,
        }
    }
    
}

impl UserState {
    pub fn new ( user_pk : Pubkey, user_data : Pubkey, transfer_lamports : u64 ) -> Self {
        UserState {
            user_pk: user_pk,
            user_data: user_data,
            user_balance: transfer_lamports,
            user_stakes: 0,
            pot_index : 0,
            card_1 : Card::default(),
            card_2 : Card::default(),
            lamports_won: 0,
        }
    }

    pub fn add_balance ( &mut self, lamports : u64 ) {
        self.user_balance = self.user_balance.checked_add(lamports).unwrap();
    }
}