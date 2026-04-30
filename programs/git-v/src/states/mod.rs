use anchor_lang::prelude::*;

#[account]
pub struct ConfigIssue {
    pub bump: u8,
    pub is_open: bool,
    pub issue_number: u32,
    pub amount: u64,
    pub end_time: i64,
    pub owner: Pubkey,
    pub created_at: i64,
    pub repo_name: Vec<u8>,
    pub username: Vec<u8>,
}
