use std::sync::Arc;

use ethers::{providers::Middleware, types::Address};
use log::warn;

use crate::contractss::{constants::*, OFC20};

pub fn create_ofc_20<M: Middleware>(
    currency: &String,
    chain: &String,
    provider: Arc<M>,
) -> Option<(OFC20<M>, Address)> {
    match currency.as_str() {
        "USDT" => match chain.as_str() {
            "BNB" => {
                let addr = ADDR_USDT_BNB.parse::<Address>().unwrap();
                let addr_for_approve = ADDR_BNB_ROUTER.parse::<Address>().unwrap();
                Some((OFC20::new(addr, provider), addr_for_approve))
            }
            "AVAX" => {
                let addr = ADDR_USDT_AVAX.parse::<Address>().unwrap();
                let addr_for_approve = ADDR_AVAX_ROUTER.parse::<Address>().unwrap();
                Some((OFC20::new(addr, provider), addr_for_approve))
            }
            "POL" => {
                let addr = ADDR_USDT_POL.parse::<Address>().unwrap();
                let addr_for_approve = ADDR_POL_ROUTER.parse::<Address>().unwrap();
                Some((OFC20::new(addr, provider), addr_for_approve))
            }
            "ARB" => {
                let addr = ADDR_USDT_ARB.parse::<Address>().unwrap();
                let addr_for_approve = ADDR_ARB_ROUTER.parse::<Address>().unwrap();

                Some((OFC20::new(addr, provider), addr_for_approve))
            }
            "OPT" => {
                let addr = ADDR_USDT_OPT.parse::<Address>().unwrap();
                let addr_for_approve = ADDR_OPT_ROUTER.parse::<Address>().unwrap();
                Some((OFC20::new(addr, provider), addr_for_approve))
            }
            "FANTOM" => {
                warn!(" Chain {} not supported ", chain);
                None
            }
            _ => {
                warn!(" Chain {} not supported ", chain);
                None
            }
        },
        "STG" => match chain.as_str() {
            "BNB" => {
                let addr = ADDR_STG_BNB.parse::<Address>().unwrap();
                Some((OFC20::new(addr, provider), addr))
            }
            "AVAX" => {
                let addr = ADDR_STG_AVAX.parse::<Address>().unwrap();
                Some((OFC20::new(addr, provider), addr))
            }
            "POL" => {
                let addr = ADDR_STG_POL.parse::<Address>().unwrap();

                Some((
                    OFC20::new(ADDR_STG_POL.parse::<Address>().unwrap(), provider),
                    addr,
                ))
            }
            "ARB" => {
                let addr = ADDR_STG_ARB.parse::<Address>().unwrap();
                Some((OFC20::new(addr, provider), addr))
            }
            "OPT" => {
                let addr = ADDR_STG_OPT.parse::<Address>().unwrap();

                Some((OFC20::new(addr, provider), addr))
            }
            "FANTOM" => {
                let addr = ADDR_STG_FANTOM.parse::<Address>().unwrap();

                Some((OFC20::new(addr, provider), addr))
            }
            _ => {
                warn!(" Chain {} not supported ", chain);
                None
            }
        },
        _ => {
            warn!(" Currency {} not supported ", currency);
            None
        }
    }
}
