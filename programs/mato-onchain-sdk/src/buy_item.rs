use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, MintTo};

use crate::{create_nft::SdkItemMeta, create_project::SdkProject, error::SdkError};

#[derive(Accounts)]
pub struct BuyItem<'info> {

    #[account(mut)]
    pub signer: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            b"project",
            project.uid.key().as_ref()
        ],
        bump = project.bump
    )]
    pub project: Account<'info,SdkProject>,

    #[account(
        mut,
        seeds = [
            b"escrow",
            project.key().as_ref()
        ],
        bump = project.escrow_bump
    )]
    pub mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub mint : AccountInfo<'info>,

    #[account(mut)]
    pub buyer_item_token_account : AccountInfo<'info>,

    #[account(mut)]
    pub buyer_payment_token_account: Account<'info,TokenAccount>,

    #[account(mut)]
    pub escrow_payment_token_account: Account<'info,TokenAccount>,

    #[account(mut)]
    pub mint_meta: Account<'info,SdkItemMeta>,

    pub token_program : AccountInfo<'info>,
}

impl <'info> BuyItem<'info> {
    pub fn handler(&mut self,items: u64) -> Result<()> {
        let meta = &mut self.mint_meta;
        let expected_payemnt = meta.price.checked_mul(items);

        let project = &self.project;

        if meta.items_count >= meta.max_items {
            return  err!(SdkError::AllItemsAreSold);
        }

        if expected_payemnt.is_some() {
            let payment = expected_payemnt.unwrap();

            if payment > self.buyer_payment_token_account.amount {
                return err!(SdkError::NotEnoughFunds);
            }

            meta.items_count += 1;

            // payment
            {
                let transfer_accs = anchor_spl::token::Transfer {
                    from: self.buyer_payment_token_account.to_account_info(),
                    to: self.escrow_payment_token_account.to_account_info(),
                    authority: self.signer.to_account_info(),
                };

                let transfer_ctx =
                    CpiContext::new(self.token_program.to_account_info(), transfer_accs);

                // check accounts mints
                {
                    if !self.buyer_payment_token_account.mint.eq(&meta.price_mint) {
                        msg!("payer's token account mint is not the one required to pay");

                        return Err(ErrorCode::Deprecated.into());
                    }

                    if !self.escrow_payment_token_account.mint.eq(&meta.price_mint) {
                        msg!("escrow payment account mint is not equal to sender mint");
                        return Err(ErrorCode::Deprecated.into());
                    }

                    if !self
                        .escrow_payment_token_account
                        .owner
                        .eq(&project.escrow)
                    {
                        msg!("escrow wallet ({}) account wrong. owner is {}. expected {}",
                        self.escrow_payment_token_account.key(),
                        self.escrow_payment_token_account
                        .owner,project.escrow);
                        return err!(SdkError::WalletAccountIsWrong);
                    }
                }
                // transfer payment
                anchor_spl::token::transfer(transfer_ctx, payment)?;
                msg!("payment success");
            }

            // mint tokens
            {
                let mint_to_accs = MintTo {
                    mint: self.mint.to_account_info(),
                    to: self.buyer_item_token_account.to_account_info(),
                    authority: self.mint_authority.to_account_info(),
                };

                let key_val = project.key();

                let seeds = &[b"escrow".as_ref(),  key_val.as_ref(), &[project.escrow_bump]];
                let signer = &[&seeds[..]];

                msg!("signer seeds with bump: {}",project.escrow_bump);

                let mint_to_ctx = CpiContext::new_with_signer(
                    self.token_program.to_account_info(),
                    mint_to_accs,
                    signer,
                );

                // mint tokens
                anchor_spl::token::mint_to(mint_to_ctx, items)?;
            }
        } else {
            return err!(SdkError::PriceError);
        }

        Ok(())
    }
}