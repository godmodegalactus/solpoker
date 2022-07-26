use crate::*;

pub const MAX_NUMBER_OF_PLAYERS : u8 = 10;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DataType {
    Unknown,
    Context,
    Game,
    User,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::Unknown
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CurrentGameState {
    NotYetStarted,
    NoCardsShown,
    ThreeCardsShown,
    FourCardsShown,
    AllCardsShown,
    GameEnded,
    CalculatingWinner,
}

impl Default for CurrentGameState {
    fn default() -> Self {
        CurrentGameState::NotYetStarted
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Suit {
    Unknown,
    Clubs,
    Dimonds,
    Hearts,
    Spades,
}

impl Default for Suit {
    fn default() -> Self {
        Suit::Unknown
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CardValue {
    Unknown,
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl Default for CardValue {
    fn default() -> Self {
        CardValue::Unknown
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum UserState {
    WaitingToStart,
    WaitingForTurn,
    WaitingForCards,
    WaitingForResponse,
    Check,
    Bid {
        amount : u64,
    },
    Fold,
    AllIn,
    Leaving,
    Left,
}

impl Default for UserState {
    fn default() -> Self {
        UserState::WaitingToStart
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum UserMoves {
    Fold,
    Check,
    Call,
    Raise {
        amount : u64
    },
}