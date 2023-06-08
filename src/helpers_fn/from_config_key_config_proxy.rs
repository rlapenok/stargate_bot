use std::sync::Arc;

use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Provider},
    signers::{Signer, Wallet},
};
use log::{info, warn};

use crate::{
    helpers_fn::{
        create_contract_check_balance_not_native_currency::create_contract_check_balance_not_native_currency_chain_destination,
        create_contract_router::create_contract_router, create_ofc_20::create_ofc_20,
        create_provider::*,
        created_contract_for_native_currency_chain::created_contract_for_native_currency_chain,
        matching_chain_to_chain_and_stargate_chain::matching_chain_to_chain_and_stargate_chain,
    },
    user_wallet_eth::user_wallet_currency::UserWalletCurrency,
};

pub fn from_config_to_user_wallets(
    config: Vec<Vec<String>>,
) -> Vec<UserWalletCurrency<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
    let mut store = Vec::new();
    info!("Started created  UserWallet's");

    config.iter().for_each(|wallet| {
        //Создание провайдеров для сети, которая отправляет и для сети, которая принимает
        let (provider_chain_sender, provider_chain_receiver) = create_provider(wallet);
        //Парсинг сети из которой отправляется транза и сети, которая принимает монеты
        let ((chain_sender, chain_receiver), (url_from, url_to)) =
            matching_chain_to_chain_and_stargate_chain(&wallet[2..=3]);
        //Контракт для чека прайса нативной монеты сети в usd
        let contract_native_currency_in_usd =
            created_contract_for_native_currency_chain(&wallet[2], provider_chain_sender.clone())
                .unwrap();
        //Создание SignerMiddleware для сети , из которой отправляется транзакция
        let wall = wallet[0]
            .trim()
            .parse::<Wallet<SigningKey>>()
            .unwrap()
            .with_chain_id(chain_sender.unwrap());
        let middleware = Arc::new(SignerMiddleware::new(provider_chain_sender, wall));
        //Создание контракта ROUTER это подойдет ко всем, т.к. те кошельки,которые гоняют эфир будут там звонить на quoteLAyerZero,
        //а другие кошельки будут звонить и quoteLayerZero  и swap
        let (router, _addr_contract_contract_for_approve) =
            create_contract_router(middleware.clone(), &wallet[2]).unwrap();
        let addr = middleware.address();
        info!("Router contract are created");
        //Матчинг на currency=ETH|Othres currency

        match wallet[1].as_str() {
            "ETH" => {
                warn!("ETH not supported");
            }
            _ => {
                //Контракт, который будет звонить на контракт монеты,которую свапнули в другую сеть (для чека баланса)
                //Добавить логи
                let contract_get_balance =
                    create_contract_check_balance_not_native_currency_chain_destination(
                        &wallet[3],
                        &wallet[1],
                        provider_chain_receiver,
                    )
                    .unwrap();
                //Контракт, который аппрувит и свапает монеты
                //Добавить логи
                let (contract_approve, contract_for_approve) =
                    create_ofc_20(&wallet[1], &wallet[2], middleware.clone()).unwrap();
                let user_wallet = UserWalletCurrency::new(
                    addr,
                    chain_receiver.unwrap(),
                    contract_approve,
                    router,
                    contract_get_balance,
                    middleware,
                    contract_native_currency_in_usd,
                    contract_for_approve,
                    url_from.unwrap(),
                    url_to.unwrap(),
                    wallet[1].clone(),
                );
                store.push(user_wallet);
            }
        }
    });

    info!("UserWallet's are created");
    store
}
