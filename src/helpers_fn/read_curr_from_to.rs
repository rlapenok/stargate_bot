use std::fs::read_to_string;

use log::info;

//
pub fn read_curr_from_to()->Vec<Vec<Vec<String>>>{
    //Чтение пути до eth_arb_opt.txt,eth_opt_arb.txt,stg_from_to.txt, usdt_from_to.txt и парсинг в private_key,config_rpoxy
    let mut currency_from_to_path=Vec::new();
    let mut private_key_config_proxy=Vec::new();
    let name_path=["ETH_FROM_TO","STG_FROM_TO","USDT_FROM_TO"];
    info!("Started reading path to eth_from_to.txt,stg_from_to.txt, usdt_from_to.txt");
      name_path.into_iter().for_each(|path|{
            let path=dotenv::var(path).unwrap().trim().to_string();
            currency_from_to_path.push(path)
      });  

    currency_from_to_path.iter().for_each(|path|{

        //Чтение из пути данных(приватный ключ \n конфиг для прокси)
        let data=read_to_string(path.trim()).unwrap()
        .split("\n")
        .map(|x|{
            x.trim().split(";").map(|wallet|wallet.to_string()).collect::<Vec<String>>()
            
        }).collect::<Vec<Vec<String>>>();
        private_key_config_proxy.push(data);

    });  
    info!("Path to eth_from_to.txt,stg_from_to.txt, usdt_from_to.txt are readed");

      private_key_config_proxy



}