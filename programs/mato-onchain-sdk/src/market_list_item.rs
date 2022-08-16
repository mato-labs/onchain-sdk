use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};

use crate::{
    create_nft::SdkItemMeta, create_project::SdkProject, error::SdkError, traits::FixedSpace,
};

#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account()]
    pub project: Box<Account<'info, SdkProject>>,

    #[account()]
    pub mint: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            b"meta",
            mint.key().as_ref()
        ],
        bump
    )]
    pub mint_meta: Box<Account<'info, SdkItemMeta>>,

    #[account(
        init,
        payer = seller,
        space = Listing::space(),
        seeds = [
            b"listing",
            mint.key().as_ref(),
            seller.key().as_ref()
        ],
        bump
    )]
    pub listing_info: Box<Account<'info, Listing>>,

    #[account(
        seeds = [
            b"market_escrow",
            project.key().as_ref()
        ],
        bump
    )]
    pub market_escrow: AccountInfo<'info>,

    #[account(
        init,
        payer = seller,
        seeds = [
            b"listing_tacc",
            listing_info.key().as_ref()
        ],
        token::mint = mint,
        token::authority = market_escrow,
        bump
    )]
    pub market_escrow_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub payment_token_mint: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::mint = mint.key(),
        associated_token::authority = seller.key(),
    )]
    pub seller_item_token_account: Account<'info, TokenAccount>,

    #[account()]
    pub system_program: Program<'info, System>,

    #[account()]
    pub token_program: AccountInfo<'info>,

    pub rent : AccountInfo<'info>

}

#[account]
#[derive(Default)]
pub struct Listing {
    pub project: Pubkey,

    pub mint: Pubkey,

    // the one who can withdraw/edit sale
    pub seller: Pubkey,

    pub sale_amount: u64,
    pub sale_price_total: u64,

    // not in use
    pub sale_expire_at: u64,

    pub payment_token_mint: Pubkey,
    pub market_escrow_token_account: Pubkey,
}

impl FixedSpace for Listing {
    fn space() -> usize {
        let pd = Listing::default();
        let serialized_object = borsh::to_vec(&pd).unwrap();

        return serialized_object.len() + 8;
    }
}

impl<'info> CreateListing<'info> {
    pub fn handler(
        &mut self,
        total_price: u64,
        amount: u64,
        _expire_at: u64, // unix timestamp
    ) -> Result<()> {
        let listing_info = &mut self.listing_info;

        if !self
            .project
            .market_escrow
            .eq(self.market_escrow.key)
        {
            return err!(SdkError::MarketEscrowError);
        }

        // transfer tokens to market escrow
        {
            let transfer_ctx = CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.seller_item_token_account.to_account_info(),
                    to: self.market_escrow_token_account.to_account_info(),
                    authority: self.seller.to_account_info(),
                },
            );

            token::transfer(transfer_ctx, amount)?;
        }

        // accs init

        listing_info.project = self.project.key();
        listing_info.mint = self.mint.key();
        listing_info.seller = self.seller.key();

        listing_info.market_escrow_token_account = self.market_escrow_token_account.key();
        listing_info.payment_token_mint = self.payment_token_mint.key();
        listing_info.market_escrow_token_account = self.market_escrow_token_account.key();

        listing_info.sale_price_total = total_price;
        listing_info.sale_amount = amount;

        listing_info.sale_expire_at = 0;

        Ok(())
    }
}
