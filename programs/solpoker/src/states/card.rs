use crate::*;
use crate::states::{ enums::Suit, enums::CardValue };

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
#[repr(C)]
pub struct Card {
    suit : Suit,
    value : CardValue,
}

impl Default for Card {
    fn default() -> Self {
        Card {
            suit: Suit::Unknown,
            value: CardValue::Unknown,
        }
    }
}