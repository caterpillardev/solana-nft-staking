use anchor_lang::prelude::*;

declare_id!("3cVdUcwFVhnmoxSZ6ciBMGxDb3YJDAWmgoVgsQPqqr8V");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
