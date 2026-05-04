use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};

use crate::{ConfigIssue, CreateIssue, CreateIssueBumps};

impl<'info> CreateIssue<'info> {
    fn create_config(
        &mut self,
        username: Vec<u8>,
        repo_name: Vec<u8>,
        issue_number: u32,
        reward_amount: u64,
        end_time: i64,
        bump: &CreateIssueBumps,
    ) -> Result<()> {
        let clock = Clock::get()?;
        self.config_account.set_inner(ConfigIssue {
            bump: bump.config_account,
            is_open: true,
            end_time,
            created_at: clock.unix_timestamp,
            reward_amount,
            owner: self.owner.key(),
            reward_mint: self.mint.key(),
            issue_number,
            username,
            repo_name,
        });
        Ok(())
    }

    fn deposite_fund(&mut self, reward_amount: u64) -> Result<()> {
        let transfer_acc = TransferChecked {
            from: self.owner_ata.to_account_info(),
            to: self.config_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.owner.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_acc);
        transfer_checked(cpi_ctx, reward_amount, self.mint.decimals)
    }
}
