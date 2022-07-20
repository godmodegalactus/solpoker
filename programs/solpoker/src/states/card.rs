
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
#[repr(C)]
pub struct Card {
    suit : Suit,
    value : CardValue,
}

impl Default for Card {
    fn default() -> Self {
        Card {
            suit: Suit::unknown,
            value: CardValue::unknown,
        }
    }
}