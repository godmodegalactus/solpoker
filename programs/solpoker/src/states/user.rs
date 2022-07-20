
#[account()]
pub struct User {
    meta_data : MetaData,
    // owner public key
    owner_pk : Pubkey,
    // base mint
    base_mint : Pubkey,
    // number of base_token in lamports
    balance_lamports : u64,
    // number of games won
    won_count : u32,
    // loose count
    lose_count : u32,
    // lamports_won 
    lamport_won : u64,
}