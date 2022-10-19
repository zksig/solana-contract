use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod esignature {
    use super::*;

    pub fn create_agreement(
        ctx: Context<CreateAgreement>,
        identifier: String,
        cid: String,
        description_cid: String,
        total_packets: u8,
    ) -> Result<Pubkey> {
        instructions::create_agreement(ctx, identifier, cid, description_cid, total_packets)
    }

    pub fn approve_agreement(ctx: Context<ApproveAgreement>) -> Result<()> {
        instructions::approve_agreement(ctx)
    }

    pub fn reject_agreement(ctx: Context<RejectAgreement>) -> Result<()> {
        instructions::reject_agreement(ctx)
    }

    pub fn create_signature_packet(
        ctx: Context<CreateSignaturePacket>,
        identifier: String,
        signer: Option<Pubkey>,
    ) -> Result<Pubkey> {
        instructions::create_signature_packet(ctx, identifier, signer)
    }

    pub fn sign_signature_packet(
        ctx: Context<SignSignaturePacket>,
        identifier: String,
    ) -> Result<()> {
        instructions::sign_signature_packet(ctx, identifier)
    }
}
