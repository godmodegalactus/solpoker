use crate::*;

#[error_code]
pub enum SolPokerErrors {
    #[msg("Game Is Full")]
    GameIsFull,
    #[msg("Cannot find user in the game")]
    CannotFindUserInGame,
    #[msg("Game not correctly initailized")]
    GameNotCorrectlyInitialized,
    #[msg("Invalid Mint")]
    InvalidMint,
    #[msg("Joining amount should be greater than small blind")]
    AmountLessThanSmallBlind,
    #[msg("Invalid Oracle")]
    InvalidOracle,
    #[msg("Last update has not passed timeout")]
    LastStateNotInTimeout,
    #[msg("Unknown State")]
    UnknownState,
    #[msg("Not Enough Players")]
    NotEnoughPlayers,
    #[msg("Not Enough Cards")]
    NotEnoughCards,
    #[msg("Invalid State")]
    InvalidState,
}

pub fn check(result : bool, error : SolPokerErrors) -> Result<()> 
{
    if result {
        Ok(())
    } else {
        Err(error.into())
    }
}

pub fn throw(error : SolPokerErrors) -> Result<()>
{
    Err(error.into())
}