use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("FqUDkQ5xq2XE7BecTN8u9R28xtLudP7FgCTC8vSLDEwL");

#[program]
pub mod e_signature {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>) -> Result<()> {
        instructions::create_profile(ctx)
    }

    pub fn create_agreement(
        ctx: Context<CreateAgreement>,
        identifier: String,
        cid: String,
        description_cid: String,
        total_packets: u8,
    ) -> Result<()> {
        instructions::create_agreement(ctx, identifier, cid, description_cid, total_packets)
    }

    pub fn create_signature_constraint(
        ctx: Context<CreateSignatureConstraint>,
        index: u8,
        identifier: String,
        signer: Option<Pubkey>,
    ) -> Result<()> {
        instructions::create_signature_constraint(ctx, index, identifier, signer)
    }

    pub fn approve_agreement(ctx: Context<ApproveAgreement>) -> Result<()> {
        instructions::approve_agreement(ctx)
    }

    pub fn reject_agreement(ctx: Context<RejectAgreement>) -> Result<()> {
        instructions::reject_agreement(ctx)
    }

    pub fn sign_signature_packet(ctx: Context<SignSignaturePacket>, index: u8) -> Result<()> {
        instructions::sign_signature_packet(ctx, index)
    }
}
