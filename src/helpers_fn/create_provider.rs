use ethers::providers::{Provider,Http, Middleware};
use log::info;
use reqwest::{Url, Proxy, Client};

use serde::{Serialize, Deserialize};

use super::matching_chain_to_rpc::matching_chain_to_rpc;

#[derive(Serialize,Deserialize)]
pub struct ProxyJson {
    pub addr: String,
    pub login: String,
    pub password: String,
}

pub fn create_provider(key_and_config_proxy:&Vec<String>)->impl Middleware{
    //Мтачинг на наличие прокси
    match key_and_config_proxy.get(3) {
        Some(config)=>{
             //Парсинг конфига прокси в ProxyJson
                let proxy_json=serde_json::from_str::<ProxyJson>(&config.trim()).unwrap();
             //Получение адреса rpc
                 let rpc=matching_chain_to_rpc(&key_and_config_proxy[1]).unwrap();
                 //Создание провайдера с прокси
            info!("Create provider with proxy:{} for:{}",rpc,&key_and_config_proxy[0]);
             create_provider_with_proxy(rpc, proxy_json)
        }
        None=>{
            //Создание провайдера без прокси
             //Получение адреса rpc
            let rpc=matching_chain_to_rpc(&key_and_config_proxy[1]).unwrap();
            info!("Create provider without proxy:{} for:{}",rpc,&key_and_config_proxy[0]);
            let provider=Provider::<Http>::try_from(rpc.trim()).unwrap();
            provider
        }
}
}



fn create_provider_with_proxy(rpc:String,proxy_json:ProxyJson)->Provider<Http>{
    let url = Url::parse(rpc.trim()).unwrap();
        let proxy = Proxy::http(&proxy_json.addr)
            .unwrap()
            .basic_auth(&proxy_json.login, &proxy_json.password);
        let client = Client::builder().proxy(proxy).build().unwrap();
        let http_provider = Http::new_with_client(url, client);
        let provider = Provider::new(http_provider);
        provider
}


