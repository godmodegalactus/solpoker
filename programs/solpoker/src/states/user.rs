use crate::*;
use crate::states::meta_data::MetaData;

#[account()]
pub struct User {
    pub meta_data : MetaData,
    // owner public key
    pub owner_pk : Pubkey,
    // base mint
    pub base_mint : Pubkey,
    // number of base_token in lamports
    pub balance_lamports : u64,
    // number of games won
    pub won_count : u32,
    // loose count
    pub lose_count : u32,
    // lamports_won 
    pub lamport_won : u64,
}

impl User {
    pub fn add_balance( &mut self, lamports : u64 ) -> Result<()> {
        self.balance_lamports = self.balance_lamports.checked_add(lamports).unwrap();
        Ok(())
    }
}