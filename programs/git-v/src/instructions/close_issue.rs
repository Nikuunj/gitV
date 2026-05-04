use anchor_lang::prelude::*;
use anchor_spl::token_interface::{close_account, transfer_checked, CloseAccount, TransferChecked};

use crate::{CloseIssue, GitVErrorCode};

impl<'info> CloseIssue<'info> {
    pub fn refund_rewards(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let owner_key = self.owner.key();
        let issue_number_bytes = self.config_account.issue_number.to_le_bytes();

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"config",
            owner_key.as_ref(),
            self.config_account.username.as_ref(),
            self.config_account.repo_name.as_ref(),
            &issue_number_bytes,
            &[self.config_account.bump],
        ]];

        require!(
            clock.unix_timestamp >= self.config_account.end_time,
            GitVErrorCode::TooEarlyToWithdraw
        );

        let tx_acc = TransferChecked {
            from: self.config_ata.to_account_info(),
            to: self.owner_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.config_account.to_account_info(),
        };

        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.to_account_info(), tx_acc, signer_seeds);

        transfer_checked(cpi_ctx, self.config_ata.amount, self.mint.decimals)
    }

    pub fn close_reward_account(&mut self) -> Result<()> {
        let owner_key = self.owner.key();
        let issue_number_bytes = self.config_account.issue_number.to_le_bytes();

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"config",
            owner_key.as_ref(),
            self.config_account.username.as_ref(),
            self.config_account.repo_name.as_ref(),
            &issue_number_bytes,
            &[self.config_account.bump],
        ]];

        let tx_acc = CloseAccount {
            account: self.config_ata.to_account_info(),
            authority: self.config_account.to_account_info(),
            destination: self.owner.to_account_info(),
        };

        let cpi_close = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            tx_acc,
            signer_seeds,
        );

        close_account(cpi_close)
    }
}
