use std::sync::Arc;

use ethers::providers::{Http, Provider};
use log::info;
use reqwest::{Client, Proxy, Url};

use serde::{Deserialize, Serialize};

use super::matching_chain_to_rpc::matching_chain_to_rpc;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyJson {
    pub addr: String,
    pub login: String,
    pub password: String,
}

pub fn create_provider(wallet: &[String]) -> (Arc<Provider<Http>>, Arc<Provider<Http>>) {
    //Мтачинг на наличие прокси
    match wallet.get(4) {
        Some(config) => {
            /*Парсинг конфига прокси в ProxyJson
            Получение адреса rpc и создание провайдера с прокси для сети из которой отправляется транзакция*/
            let proxy_json = serde_json::from_str::<ProxyJson>(config.trim()).unwrap();
            let rpc = matching_chain_to_rpc(&wallet[2]).unwrap();
            info!(
                "Create provider (chain_sender) with proxy for:{}",
                &wallet[0]
            );
            let provider_chain_sender = Arc::new(create_provider_with_proxy(rpc, proxy_json));
            /*Парсинг конфига прокси в ProxyJson
            Получение адреса rpc и создание провайдера с прокси для сети в которую отправляется транзакция*/
            let proxy_json = serde_json::from_str::<ProxyJson>(config.trim()).unwrap();
            let rpc = matching_chain_to_rpc(&wallet[3]).unwrap();
            info!(
                "Create provider (chain_receiver) with proxy for:{}",
                &wallet[0]
            );
            let provider_chain_receiver = Arc::new(create_provider_with_proxy(rpc, proxy_json));
            (provider_chain_sender, provider_chain_receiver)
        }
        None => {
            //Получение адреса rpc и создание провайдера без прокси для сети из которой отправляется транзакция
            let rpc = matching_chain_to_rpc(&wallet[2]).unwrap();
            info!(
                "Create provider (chain_sender) without proxy for:{}",
                &wallet[0]
            );
            let provider_chain_sender = Arc::new(Provider::<Http>::try_from(rpc.trim()).unwrap());
            //Получение адреса rpc и создание провайдера без прокси для сети в которой отправляется транзакция
            let rpc = matching_chain_to_rpc(&wallet[3]).unwrap();
            info!(
                "Create provider (chain_receiver) without proxy for:{}",
                &wallet[0]
            );
            let provider_chain_receiver = Arc::new(Provider::<Http>::try_from(rpc.trim()).unwrap());
            (provider_chain_sender, provider_chain_receiver)
        }
    }
}

fn create_provider_with_proxy(rpc: String, proxy_json: ProxyJson) -> Provider<Http> {
    let url = Url::parse(rpc.trim()).unwrap();
    let proxy = Proxy::http(&proxy_json.addr)
        .unwrap()
        .basic_auth(&proxy_json.login, &proxy_json.password);
    let client = Client::builder().proxy(proxy).build().unwrap();
    let http_provider = Http::new_with_client(url, client);
    Provider::new(http_provider)
}
