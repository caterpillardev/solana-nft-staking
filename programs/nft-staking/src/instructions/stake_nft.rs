#[account(
    constraint = global_pool.admin = *admin.key @StakingError::InvalidAdmin
)]
pub global_pool: Signer<'info>,


#[account (
    seeds = [GLOBAL_AUTORITY_SEED.as_ref()],
    bump
)]
 pub global_pool:Account<'info, Globalpool>,


 #[account(
    mut,
    seeds = [signer.key.as_ref(), UserPool.as_ref()],
    bump
 )]
pub user_pool: Box<Account<'info, UserPool>>,