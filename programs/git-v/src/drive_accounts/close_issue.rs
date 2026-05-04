use anchor_lang::prelude::*;


use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{ConfigIssue};

#[derive(Accounts)]
pub struct CloseIssue<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub mint: InterfaceAccount<'info, Mint>,


    #[account(
        mut,
        close = owner,
        has_one = owner,
        seeds = [
            b"config", 
            owner.key().as_ref(), 
            &config_account.username, 
            &config_account.repo_name, 
            &config_account.issue_number.to_le_bytes()
        ],
        bump = config_account.bump
    )]
    pub config_account: Account<'info, ConfigIssue>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program
        
    )]
    pub owner_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = config_account,
        associated_token::token_program = token_program
        
    )]
    pub config_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
