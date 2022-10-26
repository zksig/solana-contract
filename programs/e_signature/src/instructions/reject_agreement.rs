use crate::state::agreement::*;
use crate::state::profile::*;
use anchor_lang::prelude::*;

pub fn reject_agreement(ctx: Context<RejectAgreement>) -> Result<()> {
    ctx.accounts.agreement.reject()
}

#[derive(Accounts)]
pub struct RejectAgreement<'info> {
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
