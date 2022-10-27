use crate::state::*;
use anchor_lang::prelude::*;

pub fn sign_signature_packet(
    ctx: Context<SignSignaturePacket>,
    index: u8,
    encrypted_cid: String,
) -> Result<()> {
    ctx.accounts
        .agreement
        .add_signer()
        .and_then(|_| {
            ctx.accounts
                .constraint
                .use_constraint(ctx.accounts.signer.key())
        })
        .and_then(|_| {
            ctx.accounts.profile.add_signature();
            ctx.accounts.packet.setup_and_sign(
                ctx.accounts.agreement.key(),
                index,
                encrypted_cid,
                ctx.accounts.signer.key(),
                *ctx.bumps.get("packet").unwrap(),
            )
        })
}

#[derive(Accounts)]
#[instruction(index: u8)]
pub struct SignSignaturePacket<'info> {
    #[account(
        init,
        payer = signer,
        space = ESignaturePacket::MAXIMUM_SIZE + 8,
        seeds = [b"packet", profile.signatures_count.to_string().as_bytes(), signer.key().as_ref()],
        bump
    )]
    pub packet: Account<'info, ESignaturePacket>,

    #[account(
        mut,
        seeds = [b"profile", signer.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        mut,
        seeds = [b"constraint", index.to_string().as_bytes(), agreement.key().as_ref()],
        bump = constraint.bump
    )]
    pub constraint: Account<'info, SignatureConstraint>,

    #[account(mut)]
    pub agreement: Account<'info, Agreement>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
