use anchor_lang::prelude::*;

#[error_code]
pub enum SdkError {
    #[msg("wallet error is wrong")]
    WalletAccountIsWrong,

    #[msg("price multiplication error")]
    PriceError,

    #[msg("items are sold")]
    AllItemsAreSold,
    
    #[msg("not enough funds")]
    NotEnoughFunds,

    #[msg("object uid length exceeded")]
    ObjectIdTooLong,


    #[msg("market escrow hacked!")]
    MarketEscrowError,

    #[msg("listing info hacked!")]
    ListingInfoHacked,
}