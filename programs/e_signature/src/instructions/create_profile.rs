use crate::state::profile::*;
use anchor_lang::prelude::*;

pub fn create_profile(ctx: Context<CreateProfile>) -> Result<()> {
    ctx.accounts
        .profile
        .initialize(ctx.accounts.owner.key(), *ctx.bumps.get("profile").unwrap())
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        payer = owner,
        space = Profile::MAXIMUM_SIZE + 8,
        seeds = [b"profile", owner.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}
