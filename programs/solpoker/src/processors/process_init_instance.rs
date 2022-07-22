use crate::*;
use states::meta_data::MetaData;
use states::enums::DataType;

pub fn process( ctx: Context<InitInstance>, manager_fees_in_bps : u8 ) -> Result<()> {
    let manager_info = &mut ctx.accounts.manager_info;
    manager_info.meta_data = MetaData { 
        data_type : DataType::Manager, 
        version : 1, 
        is_initialized : true 
    };

    manager_info.manager_pk = ctx.accounts.manager.key();
    manager_info.base_mint = ctx.accounts.base_mint.key();
    manager_info.treasury_account = ctx.accounts.treasury_account.key();
    manager_info.manager_fees_in_bps = manager_fees_in_bps;

    manager_info.treasury_collected = 0;
    manager_info.count_of_games_currently_running = 0;
    Ok(())
}