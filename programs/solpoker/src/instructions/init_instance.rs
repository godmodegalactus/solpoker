use crate::states::manager;

#[derive(Accounts)]
pub struct InitInstance<'info> {
    #[account(mut)]
    pub manager : Signer<'info>,
    #[account()]
    pub base_mint : Signer<'info>,
}