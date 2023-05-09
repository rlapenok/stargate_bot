use std::sync::Arc;

use ethers::{providers::Middleware, abi::Address, prelude::FunctionCall};
use log::warn;

use crate::contractss::PriceNativeCurrencyChainInUSD;


pub fn created_contract_for_native_currency_chain<M:Middleware>(chain:&String,provider:Arc<M>)->Option<PriceNativeCurrencyChainInUSD<M>>{
    match chain.as_str() {
            "BNB"=>{
                let contract_addres=dotenv::var("BNB_USD_ADDR").unwrap().trim().parse::<Address>().unwrap();
                Some(PriceNativeCurrencyChainInUSD::new(contract_addres, provider))
            }
            "AVAX"=>{
                let contract_addres=dotenv::var("AVAX_USD_ADDR").unwrap().trim().parse::<Address>().unwrap();
                 Some(PriceNativeCurrencyChainInUSD::new(contract_addres, provider))

            }
            "POL"=>{
                let contract_addres=dotenv::var("POLYGON_USD_ADDR").unwrap().trim().parse::<Address>().unwrap();
                Some(PriceNativeCurrencyChainInUSD::new(contract_addres, provider))

                
            }
            "ARB"=>{
                let contract_addres=dotenv::var("ETH_USD_ARB_ADDR").unwrap().trim().parse::<Address>().unwrap();
                Some(PriceNativeCurrencyChainInUSD::new(contract_addres, provider))


            }
            "OPT"=>{
                let contract_addres=dotenv::var("ETH_USD_OPT_ADDR").unwrap().trim().parse::<Address>().unwrap();
                Some(PriceNativeCurrencyChainInUSD::new(contract_addres, provider))


            }
            "FANTOM"=>{
                let contract_addres=dotenv::var("FANTOM_USD_ADDR").unwrap().trim().parse::<Address>().unwrap();
                Some(PriceNativeCurrencyChainInUSD::new(contract_addres, provider))

            }
            _=>{
                warn!(" Chain {} not supported ", chain);
                None
            }
    }
}