use anchor_lang::prelude::*;

const MAX_NUMBER_OF_PLAYERS : u8 = 10;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DataType {
    manager = 0,
    game,
    user,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CurrentGameState {
    not_yet_started,
    no_cards_shown,
    three_cards_shown,
    four_cards_shown,
    all_cards_shown,
    game_ended,
    calculating_winner,
}

impl Default for CurrentGameState {
    fn default() -> Self {
        CurrentGameState::not_yet_started
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Suit {
    unknown,
    clubs,
    dimonds,
    hearts,
    spades,
}

impl Default for Suit {
    fn default() -> Self {
        Suit::unknown
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CardValue {
    unknown,
    ace,
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
    ten,
    jack,
    queen,
    king
}

impl Default for CardValue {
    fn default() -> Self {
        CardValue::unknown
    }
}