use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ConfigIssue {
    pub bump: u8,
    pub is_open: bool,
    pub issue_number: u32,
    pub end_time: i64,
    pub created_at: i64,
    pub reward_amount: u64,
    pub owner: Pubkey,
    pub reward_mint: Pubkey,
    #[max_len(100)]
    pub repo_name: Vec<u8>,
    #[max_len(50)]
    pub username: Vec<u8>,
}
