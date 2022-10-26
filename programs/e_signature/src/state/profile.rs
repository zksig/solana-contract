use anchor_lang::prelude::*;

#[account]
pub struct Profile {
    owner: Pubkey,
    pub agreements_count: u32,
    pub signatures_count: u32,
    pub bump: u8,

    reserved: [u8; 64],
}

impl Profile {
    pub const MAXIMUM_SIZE: usize = 32 + 32 + 32 + 1 + 64;

    pub fn initialize(&mut self, owner: Pubkey, bump: u8) -> Result<()> {
        self.owner = owner;
        self.agreements_count = 0;
        self.signatures_count = 0;
        self.bump = bump;

        Ok(())
    }

    pub fn add_agreement(&mut self) {
        self.agreements_count += 1;
    }

    pub fn add_signature(&mut self) {
        self.signatures_count += 1;
    }
}
