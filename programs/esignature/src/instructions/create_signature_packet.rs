use crate::state::agreement::*;
use anchor_lang::prelude::*;

pub fn create_signature_packet(
    ctx: Context<CreateSignaturePacket>,
    identifier: String,
    signer: Option<Pubkey>,
) -> Result<Pubkey> {
    match ctx.accounts.packet.setup(
        ctx.accounts.agreement.key(),
        identifier,
        signer,
        *ctx.bumps.get("packet").unwrap(),
    ) {
        Err(e) => Err(e),
        Ok(_) => Ok(ctx.accounts.packet.key()),
    }
}

#[derive(Accounts)]
#[instruction(identifier: String)]
pub struct CreateSignaturePacket<'info> {
    #[account(
        init,
        payer = originator,
        space = ESignaturePacket::MAXIMUM_SIZE + 8,
        seeds = [b"p", agreement.key().as_ref(), identifier.as_bytes()],
        bump
    )]
    pub packet: Account<'info, ESignaturePacket>,

    #[account(mut, constraint = originator.key() == agreement.originator)]
    pub agreement: Account<'info, Agreement>,

    #[account(mut)]
    pub originator: Signer<'info>,

    pub system_program: Program<'info, System>,
}
