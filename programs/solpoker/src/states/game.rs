#[account()]
pub struct Game {
    meta_data : MetaData,
    // game id
    game_id : u32,
    // oracle
    game_oracle : Pubkey,
    // base mint
    base_mint : Pubkey,
    // number which always increases
    game_number : u32,
    // small blind in lamports
    small_blind : u64,
    // maximum amount of money user can bring to the game
    maximum_bid : u64,
    // max number of player
    max_number_of_players : u8,
    // timeouts in unix time
    timeout_in_unix : u64,
    // last updated time
    last_update_time : u64,
    // current state 
    current_state: CurrentGameState,
    // pots / as pots can be splitted on allins
    pots: [u64; 10],
    // will be opened as game progresses
    card1 : Card,
    card2 : Card,
    card3 : Card,
    card4 : Card,
    card5 : Card,
}