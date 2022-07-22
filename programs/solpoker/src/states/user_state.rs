use crate::*;
use states::card::Card;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
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
}