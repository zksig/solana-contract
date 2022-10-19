use crate::state::agreement::*;
use anchor_lang::prelude::*;

pub fn create_agreement(
    ctx: Context<CreateAgreement>,
    _identifier: String,
    cid: String,
    description_cid: String,
    total_packets: u8,
) -> Result<Pubkey> {
    match ctx.accounts.agreement.setup(
        ctx.accounts.originator.key(),
        cid,
        description_cid,
        total_packets,
    ) {
        Err(e) => Err(e),
        Ok(_) => Ok(ctx.accounts.agreement.key()),
    }
}

#[derive(Accounts)]
#[instruction(identifier: String)]
pub struct CreateAgreement<'info> {
    #[account(
        init,
        payer = originator,
        space = Agreement::MAXIMUM_SIZE + 8,
        seeds = [b"a", identifier.as_bytes(), originator.key().as_ref()],
        bump
    )]
    pub agreement: Account<'info, Agreement>,

    #[account(mut)]
    pub originator: Signer<'info>,

    pub system_program: Program<'info, System>,
}
