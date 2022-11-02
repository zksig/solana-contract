use crate::state::*;
use anchor_lang::prelude::*;

pub fn initialize_signature_packet(
    ctx: Context<InitializeSignaturePacket>,
    identifier: String,
    signer: Option<Pubkey>,
) -> Result<()> {
    ctx.accounts.packet.initialize(
        ctx.accounts.agreement.key(),
        signer,
        identifier,
        *ctx.bumps.get("packet").unwrap(),
    )
}

#[derive(Accounts)]
#[instruction(identifier: String)]
pub struct InitializeSignaturePacket<'info> {
    #[account(
        init,
        payer = owner,
        space = ESignaturePacket::MAXIMUM_SIZE + 8,
        seeds = [b"packet", identifier.as_bytes(), agreement.key().as_ref()],
        bump
    )]
    pub packet: Account<'info, ESignaturePacket>,

    #[account(mut)]
    pub agreement: Account<'info, Agreement>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}
