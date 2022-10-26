use crate::errors::ZKSigError;
use anchor_lang::prelude::*;

#[account]
pub struct SignatureConstraint {
    pub agreement: Pubkey,
    index: u8,
    identifier: String,
    signer: Option<Pubkey>,
    used: bool,
    pub bump: u8,
}

impl SignatureConstraint {
    pub const MAXIMUM_SIZE: usize = 32 + 1 + 4 + 64 + 1 + 32 + 1 + 1;

    pub fn create(
        &mut self,
        agreement: Pubkey,
        index: u8,
        identifier: String,
        signer: Option<Pubkey>,
        bump: u8,
    ) {
        self.agreement = agreement;
        self.index = index;
        self.identifier = identifier;
        self.signer = signer;
        self.used = false;
        self.bump = bump;
    }

    pub fn use_constraint(&mut self, signer: Pubkey) -> Result<()> {
        require!(!self.used, ZKSigError::UsedConstraint);
        require_eq!(
            signer,
            *self.signer.get_or_insert(signer),
            ZKSigError::MismatchedSigner
        );
        self.used = true;

        Ok(())
    }
}
