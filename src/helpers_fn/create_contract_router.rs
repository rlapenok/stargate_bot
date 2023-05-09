use std::sync::Arc;

use ethers::{providers::Middleware, abi::Address};
use log::warn;

use crate::contractss::{constants::{ADDR_BNB_ROUTER, ADDR_AVAX_ROUTER, ADDR_POL_ROUTER, ADDR_ARB_ROUTER, ADDR_OPT_ROUTER, ADDR_FANTOM_ROUTER}, ROUTER};

pub fn create_contract_router<M:Middleware>(middleware:Arc<M>,chain:&String)->Option<ROUTER<M>>{

    match chain.as_str() {
        "BNB"=>{
            let address=ADDR_BNB_ROUTER.parse::<Address>().unwrap();
            Some(ROUTER::new(address, middleware))
        }
        "AVAX"=>{
            let address=ADDR_AVAX_ROUTER.parse::<Address>().unwrap();
            Some(ROUTER::new(address, middleware))

        }
        "POL"=>{
            let address=ADDR_POL_ROUTER.parse::<Address>().unwrap();
            Some(ROUTER::new(address, middleware))

            
        }
        "ARB"=>{
            let address=ADDR_ARB_ROUTER.parse::<Address>().unwrap();
            Some(ROUTER::new(address, middleware))


        }
        "OPT"=>{
            let address=ADDR_OPT_ROUTER.parse::<Address>().unwrap();
            Some(ROUTER::new(address, middleware))


        }
        "FANTOM"=>{
            let address=ADDR_FANTOM_ROUTER.parse::<Address>().unwrap();
            Some(ROUTER::new(address, middleware))

        }
        _=>{
            warn!(" Chain {} not supported ", chain);
            None
        }
}
    
}