use std::sync::Arc;

use ethers::{providers::Middleware, types::Address};
use log::warn;

use crate::contractss::{
    constants::{
        ADDR_STG_ARB, ADDR_STG_AVAX, ADDR_STG_BNB, ADDR_STG_FANTOM, ADDR_STG_OPT, ADDR_STG_POL,
        ADDR_USDT_ARB, ADDR_USDT_AVAX, ADDR_USDT_BNB, ADDR_USDT_OPT, ADDR_USDT_POL,
    },
    Get_Balance,
};

pub fn create_contract_check_balance_not_native_currency_chain_destination<M: Middleware>(
    chain_to: &String,
    currency: &String,
    provider: Arc<M>,
) -> Option<Get_Balance<M>> {
    match currency.as_str() {
        "USDT" => match chain_to.as_str() {
            "BNB" => {
                let addr = ADDR_USDT_BNB.parse::<Address>().unwrap();
                Some(Get_Balance::new(addr, provider))
            }
            "AVAX" => {
                let addr = ADDR_USDT_AVAX.parse::<Address>().unwrap();

                Some(Get_Balance::new(addr, provider))
            }
            "POL" => {
                let addr = ADDR_USDT_POL.parse::<Address>().unwrap();

                Some(Get_Balance::new(addr, provider))
            }
            "ARB" => {
                let addr = ADDR_USDT_ARB.parse::<Address>().unwrap();
                Some(Get_Balance::new(addr, provider))
            }
            "OPT" => {
                let addr = ADDR_USDT_OPT.parse::<Address>().unwrap();
                Some(Get_Balance::new(addr, provider))
            }
            "FANTOM" => {
                warn!(" Chain {} not supported  for USDT", chain_to);
                None
            }
            _ => {
                warn!(" Chain {} not supported for USDT ", chain_to);
                None
            }
        },
        "STG" => match chain_to.as_str() {
            "BNB" => {
                let addr = ADDR_STG_BNB.parse::<Address>().unwrap();
                Some(Get_Balance::new(addr, provider))
            }
            "AVAX" => {
                let addr = ADDR_STG_AVAX.parse::<Address>().unwrap();
                Some(Get_Balance::new(addr, provider))
            }
            "POL" => {
                let addr = ADDR_STG_POL.parse::<Address>().unwrap();

                Some(Get_Balance::new(addr, provider))
            }
            "ARB" => {
                let addr = ADDR_STG_ARB.parse::<Address>().unwrap();
                Some(Get_Balance::new(addr, provider))
            }
            "OPT" => {
                let addr = ADDR_STG_OPT.parse::<Address>().unwrap();

                Some(Get_Balance::new(addr, provider))
            }
            "FANTOM" => {
                let addr = ADDR_STG_FANTOM.parse::<Address>().unwrap();

                Some(Get_Balance::new(addr, provider))
            }
            _ => {
                warn!(" Chain {} not supported for STG", chain_to);
                None
            }
        },
        _ => {
            warn!(" Currency {} not supported ", currency);
            None
        }
    }
}
