use crate::state::agreement::*;
use crate::state::profile::*;
use anchor_lang::prelude::*;

pub fn approve_agreement(ctx: Context<ApproveAgreement>) -> Result<()> {
    ctx.accounts.agreement.approve()
}

#[derive(Accounts)]
pub struct ApproveAgreement<'info> {
    #[account(
        mut,
        constraint = agreement.profile == profile.key(),
    )]
    pub agreement: Account<'info, Agreement>,

    #[account(
        mut,
        seeds = [b"profile", owner.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub owner: Signer<'info>,
}
