use anchor_lang::prelude::*;

#[account]
pub struct ESignaturePacket {
    agreement: Pubkey,
    index: u8,
    encrypted_cid: String,
    signer: Pubkey,
    signed: bool,
    pub bump: u8,

    reserved: [u8; 128],
}

impl ESignaturePacket {
    pub const MAXIMUM_SIZE: usize = 32 + 1 + 4 + 64 + 32 + 1 + 1 + 128;

    pub fn setup_and_sign(
        &mut self,
        agreement: Pubkey,
        index: u8,
        encrypted_cid: String,
        signer: Pubkey,
        bump: u8,
    ) -> Result<()> {
        self.agreement = agreement.key();
        self.index = index;
        self.encrypted_cid = encrypted_cid;
        self.signer = signer;
        self.signed = true;
        self.bump = bump;

        Ok(())
    }
}
