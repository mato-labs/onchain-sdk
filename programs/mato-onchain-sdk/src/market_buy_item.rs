use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer, CloseAccount};

use crate::{create_project::SdkProject, error::SdkError, market_list_item::Listing};

#[derive(Accounts)]
pub struct BuyFromListing<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub seller: AccountInfo<'info>,

    #[account()]
    pub project: Box<Account<'info, SdkProject>>,

    #[account()]
    pub mint: AccountInfo<'info>,

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
    pub listing_info: Box<Account<'info, Listing>>,

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
    #[account(
        mut,
        // close = seller,
        seeds = [
            b"listing_tacc",
            listing_info.key().as_ref()
        ],
        bump
    )]
    pub market_escrow_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub payment_token_mint: AccountInfo<'info>,

    // payment amount added to
    #[account(mut)]
    pub seller_payment_token_account: Box<Account<'info, TokenAccount>>,

    // payment amount deducted from
    #[account(mut)]
    pub payment_buyer_token_account: Box<Account<'info, TokenAccount>>,

    // check mint
    // account item is transfered to
    #[account(mut)]
    pub buyer_item_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub token_program: AccountInfo<'info>,
}

impl<'info> BuyFromListing<'info> {
    pub fn handler(&self) -> Result<()> {
        let listing_info = &self.listing_info;
        let project = &self.project;

        {
            // validations
            //  check market escrow
            if !project.market_escrow.eq(self.market_escrow.key) {
                return err!(SdkError::MarketEscrowError);
            }

            // listing info validation
            // seller account
            if listing_info.seller != self.seller.key() {
                msg!("seller account hacked!");
                return err!(SdkError::ListingInfoHacked);
            }

            // check escrow token acc
            if listing_info.market_escrow_token_account != self.market_escrow_token_account.key() {
                msg!(
                    "market escrow token acc expected: {}. got {}",
                    listing_info.market_escrow_token_account,
                    self.market_escrow_token_account.key()
                );
                return err!(SdkError::ListingInfoHacked);
            }

            // check payment mint of payer
            if listing_info.payment_token_mint != self.payment_buyer_token_account.mint.key() {
                msg!("mint of payet token acc is not correct");
                return err!(SdkError::ListingInfoHacked);
            }

            // seller payment dest account
            if self.seller_payment_token_account.owner != listing_info.seller
                || self.seller_payment_token_account.mint.key()
                    != listing_info.payment_token_mint.key()
            {
                msg!("seller payment token account is not ok");
                return err!(SdkError::ListingInfoHacked);
            }
        }

        // transfer payment
        {
            msg!(
                "transfering payment : {} of {}",
                listing_info.sale_price_total,
                listing_info.payment_token_mint.key()
            );

            let payment_ctx = CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.payment_buyer_token_account.to_account_info(),
                    to: self.seller_payment_token_account.to_account_info(),
                    authority: self.buyer.to_account_info(),
                },
            );

            token::transfer(payment_ctx, listing_info.sale_price_total)?;

            // msg!(
            //     "transfered from {} -> {}",
            //     self.payment_buyer_token_account.owner.key(),
            //     self.seller_payment_token_account.owner.key()
            // );
        }

        let project_key = project.key();
        let seeds = &[
            b"market_escrow".as_ref(),
            project_key.as_ref(),
            &[project.market_escrow_bump],
        ];
        let market_escrow_signer_seeds = &[&seeds[..]];

        // transfer tokens to market escrow
        {
           
            let transfer_ctx = CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.market_escrow_token_account.to_account_info(),
                    to: self.buyer_item_token_account.to_account_info(),
                    authority: self.market_escrow.to_account_info(),
                },
                market_escrow_signer_seeds,
            );

            token::transfer(transfer_ctx, listing_info.sale_amount)?;

            // msg!(
            //     "transfered tokens from {} -> {}",
            //     self.market_escrow_token_account.owner.key(),
            //     self.buyer_item_token_account.owner.key(),
            // );
        }

        {
            let ca = CloseAccount {
                account: self.market_escrow_token_account.to_account_info(),
                destination: self.seller.to_account_info(),
                authority: self.market_escrow.to_account_info(),
            };
            let cpi_ctx =
                CpiContext::new_with_signer(self.token_program.to_account_info(), ca, market_escrow_signer_seeds);
            anchor_spl::token::close_account(cpi_ctx)?;
        }

        Ok(())
    }
}
