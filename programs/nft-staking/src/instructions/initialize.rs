use crate ::*;

#[derive(Accounts)]

pub struct Initialize <'info> {
    #[account(mut)]
    pub admin:Signer<'info>,


    #[account(
        init,
        space = 8 + Globalpool::DATA_SIZE,
        seeds = [GLOBAL_AUTORITY_SEED.as_ref()],
        bump, 
        payer = admin,     
    )]
    pub global_pool: Account<'info, Globalpool>,
   
    // Required for account creation

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}


impl Initialize<'_>{
    pub fn Initialize(ctx: &mut Context<Self>) -> Result<()>{
        let global_pool =&mut ctx.accounts.global_pool;
        global_pool.admin = &mut ctx.accounts.admin.key();
        Ok(())
    }
}