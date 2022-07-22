use crate::*;
use instructions::topup_account::TopupAccount;
use anchor_spl::token::{ Transfer, transfer };

pub fn process(ctx : Context<TopupAccount>, balance : u64) -> Result<()> {
    
    // transfer from user to treasury
    let accounts = Transfer {
        from: ctx.accounts.owner_token_account.to_account_info().clone(),
        to: ctx.accounts.treasury_account.to_account_info().clone(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let transfer_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info().clone(), accounts);
    transfer( transfer_ctx, balance)?;

    ctx.accounts.user.add_balance(balance)?;
    Ok(())
}