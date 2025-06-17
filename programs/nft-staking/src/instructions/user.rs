use crate ::*;

#[derive(Accounts)]

pub struct Userpool<'info>{

    #[account(mut)]
    pub user: Signer<'info>,


    #[account(
        init, 
        space = 8 + UserPool::DATA_SIZE,
        seeds =[user.key.as_ref(), USER_POOL_SEED.as_ref()],
        bump, 
        payer = user,
    )]
    pub user_pool: Account<'info, UserPool>,

    //required for account creation

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, rent>,
}

imple UserPool<'_>{
    pub fn Userpool (ctx: &mut Context<Self>) -> Result<()>{
            let user_pool = &mut ctx.accounts.user_pool;
            user_pool.user = &mut ctx.accounts.user.key();
            OK(())
    }
}