use crate::state::agreement::*;
use anchor_lang::prelude::*;

pub fn sign_signature_packet(ctx: Context<SignSignaturePacket>, identifier: String) -> Result<()> {
    match ctx.accounts.agreement.add_signer() {
        Err(e) => Err(e),
        Ok(_) => ctx.accounts.packet.sign(ctx.accounts.signer.key()),
    }
}

#[derive(Accounts)]
#[instruction(identifier: String)]
pub struct SignSignaturePacket<'info> {
    #[account(
        mut,
        seeds = [b"p", agreement.key().as_ref(), identifier.as_bytes()],
        bump = packet.bump
    )]
    pub packet: Account<'info, ESignaturePacket>,

    #[account(mut)]
    pub agreement: Account<'info, Agreement>,

    #[account(mut)]
    pub signer: Signer<'info>,
}
