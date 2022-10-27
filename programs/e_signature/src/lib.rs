use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("EqsfiqKZqSVk6r8mMBSsad6hMaYxy1pLqkfG2w6St9Yf");

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
        encrypted_cid: String,
        description_cid: String,
        total_packets: u8,
    ) -> Result<()> {
        instructions::create_agreement(
            ctx,
            identifier,
            cid,
            encrypted_cid,
            description_cid,
            total_packets,
        )
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

    pub fn sign_signature_packet(
        ctx: Context<SignSignaturePacket>,
        index: u8,
        encrypted_cid: String,
    ) -> Result<()> {
        instructions::sign_signature_packet(ctx, index, encrypted_cid)
    }
}
