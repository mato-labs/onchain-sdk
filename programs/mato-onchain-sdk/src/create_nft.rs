use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::{create_project::SdkProject, traits::FixedSpace};

#[derive(Accounts)]
#[instruction(item_id : String)]
pub struct CreateItem<'info> {

    // todo check seeds
    #[account()]
    pub project: Box<Account<'info, SdkProject>>,

    #[account(
        init,
        payer = authority,
        seeds = [
            b"meta",
            mint.key().as_ref()
        ],
        space = SdkItemMeta::space(),
        bump
    )]
    pub meta: Box<Account<'info, SdkItemMeta>>,

    // #[account(
    //     init,
    //     payer = authority,
    //     seeds = [
    //         b"alias",
    //         item_id.as_bytes(),
    //         project.uid.as_ref(),
    //     ],
    //     space = 40,
    //     bump
    // )]
    // pub meta_alias: Account<'info, SdkItemAlias>,

    #[account(
        init,
        signer,
        payer = authority,
        space = 82,
        owner = token_program.key()
    )]
    pub mint: AccountInfo<'info>,

    #[account(mut,signer)]
    pub authority: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            b"escrow",
            project.key().as_ref()
        ],
        bump = project.escrow_bump
    )]
    pub mint_authority: AccountInfo<'info>,

    pub price_mint: Box<Account<'info,Mint>>,

     // introduce new escrow for wallets ?
     #[account(
        init_if_needed,
        payer = authority,
        seeds = [
            b"payment_acc",
            price_mint.key().as_ref(),
            mint_authority.key().as_ref(),
        ],
        token::mint = price_mint,
        token::authority = mint_authority,
        bump
    )]
    pub payment_token_acc : Box<Account<'info,TokenAccount>>,

    pub token_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

// 32 + 8 
#[account]
#[derive(Default)]
pub struct SdkItemAlias {
    pub mint: Pubkey,
}

// 8+8+8+32+8+32+8 bytes
#[account]
pub struct SdkItemMeta {
    pub max_items: u64,
    pub items_count: u64,
    pub max_per_user: u64,

    // string
    // 32
    pub item_id: [u8; 32],

    // string
    // 128
    // pub resource_url: [u8; 128],

    // price
    pub price: u64,
    pub price_mint: Pubkey,

    pub inactive: bool,

}

impl Default for SdkItemMeta {
    fn default() -> Self {
        Self {
            max_items: Default::default(),
            items_count: Default::default(),
            max_per_user: Default::default(),
            item_id: Default::default(),
            // resource_url: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            price: Default::default(),
            price_mint: Default::default(),
            inactive: false,
        }
    }
}

impl FixedSpace for SdkItemMeta {
    fn space() -> usize {
        return 104
        + 1 // inactive;
    }
}
