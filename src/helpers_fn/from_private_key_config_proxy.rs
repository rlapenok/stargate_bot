use std::sync::Arc;

use ethers::{signers::{Wallet, Signer}, prelude::{k256::ecdsa::SigningKey, SignerMiddleware}};
use log::info;

use crate::{helpers_fn::{create_provider::*, matching_chain_to_chain_and_stargate_chain::matching_chain_to_chain_and_stargate_chain, created_contract_for_native_currency_chain::created_contract_for_native_currency_chain, create_contract_router_eth::create_contract_router_eth, create_contract_router::create_contract_router}};


pub fn from_config_to_user_wallets(config:Vec<Vec<String>>)->std::io::Result<Vec<i32>>{

        let mut b=vec![1];
        info!("Started created  UserWallet's");

         config.iter().for_each(|wallet|{
            //Создание провайдеров для сети, которая отправляет и для сети, которая принимает
            let (provider_chain_sender,provider_chain_receiver)=create_provider(&wallet);
            //Парсинг сети из которой отправляется транза и сети, которая принимает монеты
            let (chain_sender,chain_receiver)=matching_chain_to_chain_and_stargate_chain(&wallet[2..=3]);
            //Контракт для чека прайса нативной монеты сети в usd
            let contract_native_currency_in_usd=created_contract_for_native_currency_chain(&wallet[2], provider_chain_sender.clone()).unwrap();
            //Создание SignerMiddleware для сети , из которой отправляется транзакция
            let wall=wallet[0].trim().parse::<Wallet<SigningKey>>().unwrap().with_chain_id(chain_sender.unwrap());
            let middleware=Arc::new(SignerMiddleware::new(provider_chain_sender,wall));
            //Создание контракта ROUTER это подойдет ко всем, т.к. те кошельки,которые гоняют эфир будут там звонить на quoteLAyerZero,
            //а другие кошельки будут звонить и quoteLayerZero  и swap
            let router=create_contract_router(middleware.clone(), &wallet[2]).unwrap();
            //Матчинг на currency=ETH|Othres currency

            match wallet[1].as_str(){
               "ETH"=>{
                  //Создание контракта ETH_ROUTER через который будет проиходить свап
                  let eth_router=create_contract_router_eth(&wallet[2], middleware.clone());
               }
               _=>{
                  //Создать контракт, который будет звонить на контракт монеты,которую свапнули в другую сеть to_do!
                  //Создать контракт, который будет аппрувить токены перед отправкой to_do!
               }
            }
         });

        info!("UserWallet's are created");
        Ok(b)

}