use crate ::*;

#[account]
#[derive(Default)]

pub struct AdminPool{
    pub admin:Pubkey,
}

impl AdminPool{
    pub const DATA_SIZE :usize = 32,
}


#[account]
#[derive(Default)]

pub struct UserPool{
    pub user: Pubkey,
    pub stake_cnt: u16,
}


impl UserPool {
    pub const DATA_SIZE :usize =32 + 2; 
}


