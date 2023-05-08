use std::sync::Arc;

use ethers::{signers::{Wallet, Signer}, prelude::{k256::ecdsa::SigningKey, SignerMiddleware}};
use log::info;

use crate::{helpers_fn::{create_provider::*, matching_chain_to_chain_and_stargate_chain::matching_chain_to_chain_and_stargate_chain}, user_wallet_eth::user_wallet_eth::UserWalletETH};


pub fn from_private_key_config_proxy(private_key_config_proxy:Vec<Vec<Vec<String>>>)->std::io::Result<Vec<i32>>{

        let mut b=vec![1];
        info!("Started created  UserWallet ETH and UserWalletCurrency");
        //Парсинг всего набора кошельков
        private_key_config_proxy.iter().for_each(|vec_key_and_config_proxy|{
            //Парсинг каждого кошелька из eth_from_to_.txt, stg_from_to_txt, и т.д.
            vec_key_and_config_proxy.iter().for_each(|key_and_config_proxy|{
                //Проверка условия для создания UserWalletETH|UserWalletCurrency
                    //Создание UserWalletETh   
                 if &key_and_config_proxy[1]==&"ARB".to_owned(){   
                    //Парсинг сетей 
                    let (chain_from,chain_to_stargate)=matching_chain_to_chain_and_stargate_chain(&key_and_config_proxy[1..=2]);
                    //Создание провайдера
                    let provider=Arc::new(create_provider(&key_and_config_proxy));

                    //Создание кошелька
                    let wallet=key_and_config_proxy[0].trim().parse::<Wallet<SigningKey>>().unwrap().with_chain_id(chain_from.unwrap());
                    let addr=wallet.address();
                    let middleware=SignerMiddleware::new(provider, wallet);
                    let user_wallet_eth=UserWalletETH::new(addr, Arc::new(middleware));
                 }
                    //Создание UserWalletETh    
                 else if &key_and_config_proxy[1]==&"OPT".to_owned() {
                    //Парсинг сетей 
                    let (chain_from,chain_to_stargate)=matching_chain_to_chain_and_stargate_chain(&key_and_config_proxy[1..=2]);
                    //Создание провайдера 
                    let provider=create_provider(&key_and_config_proxy);
                    //Создание кошелька
                    let wallet=key_and_config_proxy[0].trim().parse::<Wallet<SigningKey>>().unwrap().with_chain_id(chain_from.unwrap());
                    let addr=wallet.address();
                    let middleware=SignerMiddleware::new(provider, wallet);
                    let user_wallet_eth=UserWalletETH::new(addr, Arc::new(middleware));

                 }
                    //Создание UserWalletCurrency   

                 else{
                    let provider=create_provider(&key_and_config_proxy);

                 }
            })

        });

        info!("UserWallet ETH and UserWalletCurrency as created");
        Ok(b)

}