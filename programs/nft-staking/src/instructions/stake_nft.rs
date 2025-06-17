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

let mint_metadata = &ctx.accounts.mint_metadata;
let nft_metadata = Metadata::from_account_info(mint_metadata);

if let Some(creators) =nft_metadata.data.creators {
    let mut isValid: u8 =0;
    for creator in creators {
        if creator.address.toString() == COLLECTION_ADDRESS {
            isValid= 1;
            break;
        }
    }

    require !(isValid=1, StakingError::InvalidCollection);
}