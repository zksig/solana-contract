use crate::state::agreement::*;
use crate::state::profile::*;
use anchor_lang::prelude::*;

pub fn initialize_agreement(
    ctx: Context<InitializeAgreement>,
    identifier: String,
    cid: String,
    encrypted_cid: String,
    description_cid: String,
    total_packets: u8,
) -> Result<()> {
    ctx.accounts.profile.add_agreement();
    ctx.accounts.agreement.initialize(
        ctx.accounts.profile.key(),
        identifier,
        cid,
        encrypted_cid,
        description_cid,
        total_packets,
    )
}

#[derive(Accounts)]
#[instruction(identifier: String)]
pub struct InitializeAgreement<'info> {
    #[account(
        init,
        payer = owner,
        space = Agreement::MAXIMUM_SIZE + 8,
        seeds = [b"agreement", profile.agreements_count.to_string().as_bytes(), profile.key().as_ref()],
        bump
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

    pub system_program: Program<'info, System>,
}
