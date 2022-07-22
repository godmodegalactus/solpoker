// this is the manager of whole program
use crate::states::meta_data::MetaData;
use crate::*;

#[account()]
pub struct Manager {
    pub meta_data: MetaData,
    // public key of manager
    pub manager_pk : Pubkey,
    // game count
    pub count_of_games_currently_running : u32,
    // base mint
    pub base_mint : Pubkey,
    // treasury account in base mint / controlled by PDA
    pub treasury_account : Pubkey,
    // manager fees in bps
    pub manager_fees_in_bps : u8,
    // treasury collected
    pub treasury_collected : u64,
}