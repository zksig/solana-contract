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

    pub fn initialize_agreement(
        ctx: Context<InitializeAgreement>,
        identifier: String,
        cid: String,
        encrypted_cid: String,
        description_cid: String,
        total_packets: u8,
    ) -> Result<()> {
        instructions::initialize_agreement(
            ctx,
            identifier,
            cid,
            encrypted_cid,
            description_cid,
            total_packets,
        )
    }

    pub fn approve_agreement(ctx: Context<ApproveAgreement>) -> Result<()> {
        instructions::approve_agreement(ctx)
    }

    pub fn reject_agreement(ctx: Context<RejectAgreement>) -> Result<()> {
        instructions::reject_agreement(ctx)
    }

    pub fn initialize_signature_packet(
        ctx: Context<InitializeSignaturePacket>,
        identifier: String,
        signer: Option<Pubkey>,
    ) -> Result<()> {
        instructions::initialize_signature_packet(ctx, identifier, signer)
    }

    pub fn sign_signature_packet(
        ctx: Context<SignSignaturePacket>,
        identifier: String,
        signature: [u8; 64],
        encrypted_cid: String,
        owner: Pubkey,
    ) -> Result<()> {
        instructions::sign_signature_packet(ctx, owner, &signature.as_slice(), encrypted_cid)
    }
}
