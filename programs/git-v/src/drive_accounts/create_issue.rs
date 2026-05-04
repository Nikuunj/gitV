use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{drive_accounts, ConfigIssue};

#[derive(Accounts)]
#[instruction(username: Vec<u8>, reponame: Vec<u8>)]
pub struct CreateIssue<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub mint: InterfaceAccount<'info, Mint>,


    #[account(
        init,
        payer = owner,
        space = ConfigIssue::INIT_SPACE + ConfigIssue::DISCRIMINATOR.len(),
        seeds = [b"config", owner.key().as_ref(), &username, &reponame],
        bump
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
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = config_account,
        associated_token::token_program = token_program
        
    )]
    pub issue_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
