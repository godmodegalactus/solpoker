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
    #[msg("InvalidOracle")]
    InvalidOracle,
}

pub fn check(result : bool, error : SolPokerErrors) -> Result<()> 
{
    if result {
        Ok(())
    } else {
        Err(error.into())
    }
}