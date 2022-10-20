use crate::state::agreement::*;
use anchor_lang::prelude::*;

pub fn reject_agreement(ctx: Context<RejectAgreement>) -> Result<()> {
    ctx.accounts.agreement.reject()
}

#[derive(Accounts)]
pub struct RejectAgreement<'info> {
    #[account(mut, constraint = agreement.originator == originator.key())]
    pub agreement: Account<'info, Agreement>,

    #[account(mut)]
    pub originator: Signer<'info>,
}
