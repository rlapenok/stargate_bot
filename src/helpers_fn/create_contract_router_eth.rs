use std::sync::Arc;

use ethers::{providers::Middleware, types::Address};

use crate::contractss::{constants::{ARB_ETH_ROUTER, OPT_ETH_ROUTER}, ETH_ROUTER};

pub fn create_contract_router_eth<M:Middleware>(chain:&String,middleware:Arc<M>)->ETH_ROUTER<M>{

    if chain.as_str()=="ARB"{

        let address=ARB_ETH_ROUTER.parse::<Address>().unwrap();
        ETH_ROUTER::new(address, middleware)
    }

    else{
        let address=OPT_ETH_ROUTER.parse::<Address>().unwrap();
        ETH_ROUTER::new(address, middleware)

    }

}