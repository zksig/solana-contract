use crate::errors::ZKSigError;
use anchor_lang::prelude::*;

#[account]
pub struct ESignaturePacket {
    agreement: Pubkey,
    signer: Option<Pubkey>,
    identifier: String,
    encrypted_cid: Option<String>,
    signed: bool,
    pub bump: u8,

    reserved: [u8; 128],
}

impl ESignaturePacket {
    pub const MAXIMUM_SIZE: usize = 32 + 1 + 32 + 1 + 4 + 64 + 1 + 4 + 64 + 1 + 1 + 128;

    pub fn initialize(
        &mut self,
        agreement: Pubkey,
        signer: Option<Pubkey>,
        identifier: String,
        bump: u8,
    ) -> Result<()> {
        self.agreement = agreement.key();
        self.signer = signer;
        self.identifier = identifier;
        self.encrypted_cid = None;
        self.signed = false;
        self.bump = bump;

        Ok(())
    }

    pub fn verify_signature(
        &self,
        verify_data: &[u8],
        owner: Pubkey,
        signature: &[u8],
    ) -> Result<()> {
        // According to this layout used by the Ed25519Program
        // https://github.com/solana-labs/solana-web3.js/blob/master/src/ed25519-program.ts#L33

        // "Deserializing" byte slices

        let num_signatures = &[verify_data[0]]; // Byte  0
        let padding = &[verify_data[1]]; // Byte  1
        let signature_offset = &verify_data[2..=3]; // Bytes 2,3
        let signature_instruction_index = &verify_data[4..=5]; // Bytes 4,5
        let public_key_offset = &verify_data[6..=7]; // Bytes 6,7
        let public_key_instruction_index = &verify_data[8..=9]; // Bytes 8,9
        let message_data_offset = &verify_data[10..=11]; // Bytes 10,11
        let message_data_size = &verify_data[12..=13]; // Bytes 12,13
        let message_instruction_index = &verify_data[14..=15]; // Bytes 14,15

        let data_pubkey = &verify_data[16..16 + 32]; // Bytes 16..16+32
        let data_sig = &verify_data[48..48 + 64]; // Bytes 48..48+64
        let data_msg = &verify_data[112..]; // Bytes 112..end

        // Expected values

        let pubkey = owner.to_bytes();
        let msg = format!("{} {}", self.identifier, self.agreement);

        let exp_public_key_offset: u16 = 16; // 2*u8 + 7*u16
        let exp_signature_offset: u16 = exp_public_key_offset + 32;
        let exp_message_data_offset: u16 = exp_signature_offset + 64;
        let exp_num_signatures: u8 = 1;
        let exp_message_data_size: u16 = msg.as_bytes().len().try_into().unwrap();

        // Header and Arg Checks

        // Header
        if num_signatures != &exp_num_signatures.to_le_bytes()
            || padding != &[0]
            || signature_offset != &exp_signature_offset.to_le_bytes()
            || signature_instruction_index != &u16::MAX.to_le_bytes()
            || public_key_offset != &exp_public_key_offset.to_le_bytes()
            || public_key_instruction_index != &u16::MAX.to_le_bytes()
            || message_data_offset != &exp_message_data_offset.to_le_bytes()
            || message_data_size != &exp_message_data_size.to_le_bytes()
            || message_instruction_index != &u16::MAX.to_le_bytes()
        {
            return Err(ZKSigError::SignatureVerificationError.into());
        }

        // Arguments
        if data_pubkey != pubkey || data_msg != msg.as_bytes() || data_sig != signature {
            return Err(ZKSigError::SignatureVerificationError.into());
        }

        Ok(())
    }

    pub fn sign(&mut self, signer: Pubkey, encrypted_cid: String) -> Result<()> {
        require_keys_eq!(
            *self.signer.get_or_insert(signer),
            signer,
            ZKSigError::MismatchedSigner
        );

        self.encrypted_cid = Some(encrypted_cid);
        self.signed = true;

        Ok(())
    }
}
