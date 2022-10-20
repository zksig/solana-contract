use crate::errors::ZKSigError;
use anchor_lang::prelude::*;

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
