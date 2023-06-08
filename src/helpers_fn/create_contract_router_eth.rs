use std::sync::Arc;

use ethers::{providers::Middleware, types::Address};
use log::info;

use crate::contractss::{
    constants::{ARB_ETH_ROUTER, OPT_ETH_ROUTER},
    Eth_Router,
};

pub fn create_contract_router_eth<M: Middleware>(chain: &str, middleware: Arc<M>) -> Eth_Router<M> {
    info!("Started creating Router_ETH contract");
    if chain == "ARB" {
        let address = ARB_ETH_ROUTER.parse::<Address>().unwrap();
        Eth_Router::new(address, middleware)
    } else {
        let address = OPT_ETH_ROUTER.parse::<Address>().unwrap();
        Eth_Router::new(address, middleware)
    }
}
