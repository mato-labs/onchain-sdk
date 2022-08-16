use anchor_lang::prelude::*;

use crate::traits::FixedSpace;

#[derive(Accounts)]
pub struct CreateProject<'info> {
    
    // its not an account, just unique id
    pub uid: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account()]
    pub authority: AccountInfo<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [
            b"project",
            uid.key().as_ref()
        ],
        space = SdkProject::space(),
        bump
    )]
    pub project: Box<Account<'info,SdkProject>>,

    // todo check bumps
    #[account(
        seeds = [
            b"escrow",
            project.key().as_ref()
        ],
        bump
    )]
    pub escrow: AccountInfo<'info>,


    // todo provide bumps to check
    #[account(
        seeds = [
            b"market_escrow",
            project.key().as_ref()
        ],
        bump
    )]
    pub market_escrow: AccountInfo<'info>,

    pub rent_program: AccountInfo<'info>,
    pub system_program : AccountInfo<'info>,

}

#[account]
#[derive(Default)]
pub struct SdkProject {

    pub authority : Pubkey,
    // pub payments_authority: Pubkey,

    // holds mints, payments
    pub escrow: Pubkey,

    // holds items on market
    pub market_escrow: Pubkey,

    pub uid: Pubkey,

    pub bump: u8,
    pub escrow_bump: u8,
    pub market_escrow_bump: u8,


    // payments
    pub secondary_sells_tax_percent_basis_points : u16,

    // project subscription level
    pub subscription_level: u8,
}

impl FixedSpace for SdkProject {
    fn space() -> usize {
        return 106  + 40; //market escrow + bump 
    }
}
