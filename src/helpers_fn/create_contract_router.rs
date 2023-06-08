use std::sync::Arc;

use ethers::{abi::Address, providers::Middleware};
use log::{info, warn};

use crate::contractss::{constants::*, Router};

pub fn create_contract_router<M: Middleware>(
    middleware: Arc<M>,
    chain: &String,
) -> Option<(Router<M>, Address)> {
    info!("Started creating Router contract");
    match chain.as_str() {
        "BNB" => {
            let address = ADDR_BNB_ROUTER.parse::<Address>().unwrap();
            Some((Router::new(address, middleware), address))
        }
        "AVAX" => {
            let address = ADDR_AVAX_ROUTER.parse::<Address>().unwrap();
            Some((Router::new(address, middleware), address))
        }
        "POL" => {
            let address = ADDR_POL_ROUTER.parse::<Address>().unwrap();
            Some((Router::new(address, middleware), address))
        }
        "ARB" => {
            let address = ADDR_ARB_ROUTER.parse::<Address>().unwrap();
            Some((Router::new(address, middleware), address))
        }
        "OPT" => {
            let address = ADDR_OPT_ROUTER.parse::<Address>().unwrap();
            Some((Router::new(address, middleware), address))
        }
        "FANTOM" => {
            let address = ADDR_FANTOM_ROUTER.parse::<Address>().unwrap();
            Some((Router::new(address, middleware), address))
        }
        _ => {
            warn!(" Chain {} not supported ", chain);
            None
        }
    }
}
