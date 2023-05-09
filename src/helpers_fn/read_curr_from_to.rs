use std::fs::read_to_string;

use log::info;

//
pub fn read_curr_from_to()->Vec<Vec<String>>{
    //Чтение пути до config.txt
    let path=dotenv::var("CONFIG").unwrap();
    info!("Started reading path to eth_from_to.txt,stg_from_to.txt, usdt_from_to.txt");
        //Чтение из пути данных(приватный ключ \n конфиг для прокси)
        let data=read_to_string(path.trim()).unwrap()
        .split("\n")
        .map(|x|{
            x.trim().split(";").map(|wallet|wallet.trim().to_string()).collect::<Vec<String>>()
            
        }).collect::<Vec<Vec<String>>>();

    info!("Data from config.txt are readed. Was reaaded {} wallets",data.len());

      data



}