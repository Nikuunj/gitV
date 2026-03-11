use anchor_lang::prelude::*;

declare_id!("rq714PcxGJJetbJNzqnv713TqVy2iTMfZBdT8LHBhgX");

#[program]
pub mod git_v {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
