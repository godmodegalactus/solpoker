// this is the manager of whole program

#[account()]
pub struct manager {
    meta_data: MetaData,
    // public key of manager
    manager_pk : Pubkey,
    // game count
    count_of_games_currently_running : u32,
    // base mint
    base_mint : Pubkey,
    // treasury account in base mint / controlled by PDA
    treasury_account : Pubkey,
    // manager fees in bps
    manager_fees : u8,
}