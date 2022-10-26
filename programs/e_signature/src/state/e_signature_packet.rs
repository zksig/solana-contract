use crate::errors::ZKSigError;
use anchor_lang::prelude::*;

#[account]
pub struct ESignaturePacket {
    agreement: Pubkey,
    index: u8,
    signer: Pubkey,
    signed: bool,
    pub bump: u8,
}

impl ESignaturePacket {
    pub const MAXIMUM_SIZE: usize = 32 + 1 + 32 + 1 + 1;

    pub fn setup(&mut self, agreement: Pubkey, index: u8, signer: Pubkey, bump: u8) -> Result<()> {
        self.agreement = agreement.key();
        self.index = index;
        self.bump = bump;
        self.signer = signer;
        self.signed = false;

        Ok(())
    }

    pub fn setup_and_sign(
        &mut self,
        agreement: Pubkey,
        index: u8,
        signer: Pubkey,
        bump: u8,
    ) -> Result<()> {
        self.agreement = agreement.key();
        self.index = index;
        self.bump = bump;
        self.signer = signer;
        self.signed = true;

        Ok(())
    }

    pub fn sign(&mut self, signer: Pubkey) -> Result<()> {
        require_eq!(
            self.signer.key(),
            signer.key(),
            ZKSigError::MismatchedSigner
        );

        self.signed = true;
        Ok(())
    }
}
