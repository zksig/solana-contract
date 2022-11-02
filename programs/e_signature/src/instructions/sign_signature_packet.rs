use crate::state::*;
use anchor_lang::{
    prelude::*,
    solana_program::{
        instruction::Instruction,
        sysvar::instructions::{load_instruction_at_checked, ID as IX_ID},
    },
};

pub fn sign_signature_packet(
    ctx: Context<SignSignaturePacket>,
    owner: Pubkey,
    signature: &[u8],
    encrypted_cid: String,
) -> Result<()> {
    let ix: Instruction = load_instruction_at_checked(0, &ctx.accounts.ix_sysvar)?;

    ctx.accounts
        .packet
        .verify_signature(&ix.data, owner, signature)
        .and_then(|_| ctx.accounts.agreement.add_signer())
        .and_then(|_| {
            ctx.accounts.profile.add_signature();
            ctx.accounts
                .packet
                .sign(ctx.accounts.signer.key(), encrypted_cid)
        })
}

#[derive(Accounts)]
#[instruction(identifier: String)]
pub struct SignSignaturePacket<'info> {
    #[account(
        seeds = [b"packet", identifier.as_bytes(), agreement.key().as_ref()],
        bump
    )]
    pub packet: Account<'info, ESignaturePacket>,

    #[account(
        mut,
        seeds = [b"profile", signer.key().as_ref()],
        bump = profile.bump
    )]
    pub profile: Account<'info, Profile>,

    #[account(mut)]
    pub agreement: Account<'info, Agreement>,

    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: System address
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
