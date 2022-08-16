use anchor_lang::prelude::*;

pub mod buy_item;
pub mod create_nft;
pub mod create_project;
pub mod error;
pub mod traits;
pub mod market_buy_item;
pub mod market_list_item;

use buy_item::*;
use create_nft::*;
use create_project::*;
use error::SdkError;

use market_buy_item::*;
use market_list_item::*;

declare_id!("GSdkVPb9aMMY43TNcHeocHvC1KCYxWiTs2ey79hKMsYN");

#[program]
pub mod gamesdk {

    use anchor_spl::token::{InitializeMint, MintTo};

    use super::*;

    pub fn market_list(
        ctx: Context<CreateListing>,
        total_price: u64, 
        amount: u64,
        expire_at: u64
    ) -> Result<()>{
        ctx.accounts.handler(total_price, amount, expire_at)
    }

    pub fn market_buy(
        ctx: Context<BuyFromListing>
    ) -> Result<()>{
        ctx.accounts.handler()
    }

    pub fn buy_game_item(
        ctx: Context<BuyItem>, 
        items: u64
    ) -> Result<()> {
        ctx.accounts.handler(items)
    }

    pub fn create_game_project(
        ctx: Context<CreateProject>, 
        project_bump: u8, 
        escrow_bump : u8,
        market_escrow_bump: u8
    ) -> Result<()> {

        // todo : handle payment

        let new_project = &mut ctx.accounts.project;

        new_project.uid = ctx.accounts.uid.key();

        // project addresses
        new_project.authority = ctx.accounts.authority.key();
        new_project.escrow = ctx.accounts.escrow.key();
        new_project.market_escrow = ctx.accounts.market_escrow.key();
        
        // bumps
        new_project.bump = project_bump;
        new_project.escrow_bump = escrow_bump;
        new_project.market_escrow_bump = market_escrow_bump;

        Ok(())
    }

    pub fn create_item(
        ctx: Context<CreateItem>,
        item_id : String,
        max_items: u64,
        max_per_user: u64,
        price_per_item: u64,
        // resource_url: Vec<[u8;32]>,
    ) -> Result<()> {

        msg!("initializing game object : {}",item_id);

        let init_mint_accs = InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };

        let init_mint_ctx =
            CpiContext::new(ctx.accounts.token_program.to_account_info(), init_mint_accs);

        anchor_spl::token::initialize_mint(init_mint_ctx, 0, &ctx.accounts.mint_authority.key(), None)?;

        // let mut alias = &mut ctx.accounts.meta_alias;
        // alias.mint = ctx.accounts.mint.key().clone();

        let meta = &mut ctx.accounts.meta;

        // usage limits
        meta.max_items  = max_items;
        meta.max_per_user = max_per_user;
        meta.items_count = 0;

        // price
        meta.price = price_per_item;
        meta.price_mint = ctx.accounts.price_mint.key();

        // game object id
        if item_id.len() > 32  {
            return err!(SdkError::ObjectIdTooLong);
        }

        let mut idx = 0;
        for it in item_id.as_bytes() {
            meta.item_id[idx] = *it;
            idx += 1;
        }

        // resource url
        // meta.resource_url[0] = resource_url[0];

        Ok(())
    }

}
