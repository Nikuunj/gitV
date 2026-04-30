use anchor_lang::prelude::*;
use rust_decimal::Decimal;
use switchboard_on_demand::{
    on_demand::accounts::pull_feed::PullFeedAccountData, prelude::rust_decimal,
};

declare_id!("Sg4bheXXXdMrcTmsSr3d2bQZhmGmBeFJCJYtkXst3Hp");

pub mod drive_accounts;
pub mod states;

pub use drive_accounts::*;
pub use states::*;

#[program]
pub mod git_v {

    use super::*;

    pub fn reed_feed(ctx: Context<ReadPrice>) -> Result<()> {
        let data_slice = ctx.accounts.feed.data.borrow();

        let feed = PullFeedAccountData::parse(data_slice).unwrap();

        let price: Decimal = feed
            .get_value(
                Clock::get()?.slot,
                /*max_stale_slots=*/ 100,
                /*min_samples=*/ 3,
                /*only_positive=*/ true,
            )
            .unwrap();

        msg!("SOL/USD price: {}", price);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ConsumeDataAccounts {}

#[derive(Accounts)]
pub struct ReadPrice<'info> {
    /// CHECK: This is a Switchboard pull feed account, validated by parsing its data
    #[account(mut)]
    pub feed: AccountInfo<'info>,
}
