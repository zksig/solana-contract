use crate::errors::ZKSigError;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
enum AgreementStatus {
    PENDING,
    COMPLETE,
    APPROVED,
    REJECTED,
}

#[account]
pub struct Agreement {
    pub originator: Pubkey,
    cid: String,
    description_cid: String,
    status: AgreementStatus,
    signed_packets: u8,
    total_packets: u8,
}

impl Agreement {
    pub const MAXIMUM_SIZE: usize = 32 + 4 + 64 + 4 + 64 + 1 + 1 + 1;

    pub fn setup(
        &mut self,
        originator: Pubkey,
        cid: String,
        description_cid: String,
        total_packets: u8,
    ) -> Result<()> {
        self.originator = originator;
        self.cid = cid;
        self.description_cid = description_cid;
        self.status = AgreementStatus::PENDING;
        self.signed_packets = 0;
        self.total_packets = total_packets;

        Ok(())
    }

    pub fn add_signer(&mut self) -> Result<()> {
        require!(
            self.status == AgreementStatus::PENDING,
            ZKSigError::NonPendingAgreement,
        );

        self.signed_packets += 1;
        if self.signed_packets == self.total_packets {
            self.status = AgreementStatus::COMPLETE
        }

        Ok(())
    }

    pub fn approve(&mut self) -> Result<()> {
        require!(
            self.status == AgreementStatus::COMPLETE,
            ZKSigError::NonPendingAgreement
        );

        self.status = AgreementStatus::APPROVED;

        Ok(())
    }

    pub fn reject(&mut self) -> Result<()> {
        require!(
            self.status == AgreementStatus::COMPLETE,
            ZKSigError::NonPendingAgreement
        );

        self.status = AgreementStatus::REJECTED;

        Ok(())
    }
}

#[account]
pub struct ESignaturePacket {
    agreement: Pubkey,
    identifier: String,
    signer: Option<Pubkey>,
    signed: bool,
    pub bump: u8,
}

impl ESignaturePacket {
    pub const MAXIMUM_SIZE: usize = 32 + 4 + 32 + 32 + 1 + 1;

    pub fn setup(
        &mut self,
        agreement: Pubkey,
        identifier: String,
        signer: Option<Pubkey>,
        bump: u8,
    ) -> Result<()> {
        self.agreement = agreement.key();
        self.identifier = identifier;
        self.bump = bump;
        self.signer = signer;
        self.signed = false;

        Ok(())
    }

    pub fn sign(&mut self, signer: Pubkey) -> Result<()> {
        require_eq!(
            self.signer.get_or_insert(signer).key(),
            signer.key(),
            ZKSigError::MismatchedSigner
        );

        self.signed = true;
        Ok(())
    }
}
