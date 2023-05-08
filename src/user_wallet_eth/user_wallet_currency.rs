use ethers::types::{Address, H160, Chain};
use crate::traits::trait_for_user_wallet::ForUserWallet;

#[derive(Debug)]
pub struct UserWalletCurrency{
    addr:Address,
    private_key:H160,
    chain_from:Chain,
    chain_to_stargate:u16
}

impl ForUserWallet for UserWalletCurrency{
    fn print(&self) {
        println!("{:?}",self);
    }
}