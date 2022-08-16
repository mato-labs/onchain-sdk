use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::{create_project::SdkProject,market_list_item::Listing};

#[derive(Accounts)]
pub struct CloseListing<'info> {
    
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account()]
    pub project: Account<'info, SdkProject>,

    #[account()]
    pub mint : AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            b"listing",
            mint.key().as_ref(),
            seller.key().as_ref()
        ],
        bump,
        close = seller
    )]
    pub listing_info : Account<'info,Listing>,

    // todo check if its the market escrow of project
    // owner of token account
    #[account(
        seeds = [
            b"market_escrow",
            project.key().as_ref()
        ],
        bump = project.market_escrow_bump
    )]
    pub market_escrow: AccountInfo<'info>,

    // mint check
    // account item is transfered from
    #[account(mut)]
    pub market_escrow_token_account : Account<'info,TokenAccount>,

    // check mint
    // account item is transfered to
    #[account(mut)]
    pub buyer_item_token_account: Account<'info,TokenAccount>,
}