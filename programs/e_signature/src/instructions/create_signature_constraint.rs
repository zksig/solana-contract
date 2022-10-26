use crate::state::agreement::*;
use crate::state::profile::*;
use crate::state::SignatureConstraint;
use anchor_lang::prelude::*;

pub fn create_signature_constraint(
    ctx: Context<CreateSignatureConstraint>,
    index: u8,
    identifier: String,
    signer: Option<Pubkey>,
) -> Result<()> {
    ctx.accounts.constraint.create(
        ctx.accounts.agreement.key(),
        index,
        identifier,
        signer,
        *ctx.bumps.get("constraint").unwrap(),
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct CreateSignatureConstraint<'info> {
    #[account(
        init,
        payer = owner,
        space = SignatureConstraint::MAXIMUM_SIZE + 8,
        seeds = [b"constraint", index.to_string().as_bytes(), agreement.key().as_ref()],
        bump
    )]
    pub constraint: Account<'info, SignatureConstraint>,

    #[account(constraint = agreement.profile == profile.key())]
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
