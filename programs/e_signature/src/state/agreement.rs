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
    pub profile: Pubkey,
    identifier: String,
    cid: String,
    encrypted_cid: String,
    description_cid: String,
    status: AgreementStatus,
    signed_packets: u8,
    total_packets: u8,

    reserved: [u8; 128],
}

impl Agreement {
    pub const MAXIMUM_SIZE: usize = 32 + (4 + 64) * 4 + 1 + 1 + 1 + 128;

    pub fn setup(
        &mut self,
        profile: Pubkey,
        identifier: String,
        cid: String,
        encrypted_cid: String,
        description_cid: String,
        total_packets: u8,
    ) -> Result<()> {
        self.profile = profile;
        self.identifier = identifier;
        self.cid = cid;
        self.encrypted_cid = encrypted_cid;
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
