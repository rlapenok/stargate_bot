use std::sync::Arc;

use crate::{
    contractss::{Get_Balance, PriceNativeCurrencyChainInUSD, Router, OFC20},
    traits::trait_for_user_wallets::ForUserWallet,
};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, H160},
};

#[derive(Debug)]
pub struct UserWalletCurrency<M: Middleware> {
    pub addr: Address,
    pub chain_to_stargate: u16,
    pub approve_contract: Arc<OFC20<M>>,
    pub router_contract: Arc<Router<M>>,
    pub get_balance_contract: Get_Balance<Provider<Http>>,
    pub middleware: Arc<M>,
    pub contract: PriceNativeCurrencyChainInUSD<Provider<Http>>,
    pub contract_for_approve: H160,
    pub url_from: String,
    pub url_to: String,
    pub currency: String,
}

impl<M: Middleware + 'static> UserWalletCurrency<M> {
    pub fn new(
        addr: Address,
        chain_to_stargate: u16,
        approve_contract: OFC20<M>,
        router_contract: Router<M>,
        get_balance_contract: Get_Balance<Provider<Http>>,
        middleware: Arc<M>,
        contract: PriceNativeCurrencyChainInUSD<Provider<Http>>,
        contract_for_approve: H160,
        url_from: String,
        url_to: String,
        currency: String,
    ) -> Self {
        Self {
            addr,
            chain_to_stargate,
            approve_contract: Arc::new(approve_contract),
            router_contract: Arc::new(router_contract),
            get_balance_contract,
            middleware,
            contract,
            contract_for_approve,
            url_from,
            url_to,
            currency,
        }
    }
}

impl<M: Middleware> ForUserWallet for UserWalletCurrency<M> {
    fn print(&self) {
        println!("{:?}", self);
    }
}
