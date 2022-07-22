use crate::*;

#[error_code]
pub enum SolPokerErrors {
    #[msg("Game Is Full")]
    GameIsFull,
    #[msg("Cannot find user in the game")]
    CannotFindUserInGame,
}