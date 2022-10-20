use crate::state::agreement::*;
use anchor_lang::prelude::*;

pub fn approve_agreement(ctx: Context<ApproveAgreement>) -> Result<()> {
    ctx.accounts.agreement.approve()
}

#[derive(Accounts)]
pub struct ApproveAgreement<'info> {
    #[account(mut, constraint = agreement.originator == originator.key())]
    pub agreement: Account<'info, Agreement>,

    #[account(mut)]
    pub originator: Signer<'info>,
}
