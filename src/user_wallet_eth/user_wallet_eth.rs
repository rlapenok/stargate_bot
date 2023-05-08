use std::sync::Arc;

use ethers::{types::{Address, H160}, prelude::SignerMiddleware, providers::{Middleware, Provider}, signers::Signer};

use crate::traits::trait_for_user_wallet::ForUserWallet;

#[derive(Debug)]
pub struct UserWalletETH<M:Middleware,S:Signer>{
    addr:Address,
    middleware:Arc<SignerMiddleware<Arc<M>,S>>,
}


impl <M:Middleware,S:Signer> UserWalletETH<M,S>{
    pub fn new(addr:Address,middleware:Arc<SignerMiddleware<M,S>>){}
}


impl<M:Middleware,S:Signer> ForUserWallet for UserWalletETH<M,S>{
    fn print(&self) {
        println!("{:?}",self);
    }
}