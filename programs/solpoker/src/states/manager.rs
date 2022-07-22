// this is the manager of whole program
use crate::states::meta_data::MetaData;
use crate::*;

#[account()]
pub struct Manager {
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
    // treasury collected
    treasury_collected : u64,
}