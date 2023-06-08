#![warn(unreachable_patterns)]
//ProxyJson { addr: "asda", login: "Adasda", password: "adsasdasd" }
use ethers::{
    abi::AbiEncode,
    prelude::{ContractCall, FunctionCall},
};
use std::{
    ops::{Div, Mul},
    sync::Arc,
    time::Duration,
};

use ethers::{
    providers::Middleware,
    types::{Address, Bytes, U256},
    utils::format_units,
};
use log::{debug, info, warn};

use crate::{
    contractss::LzTxObj,
    traits::trait_for_user_wallets::{MyMiddleware, TypeContract},
};

use super::user_wallet_currency::UserWalletCurrency;
#[async_trait::async_trait]
impl<M: Middleware + 'static> MyMiddleware<M> for UserWalletCurrency<M> {
    //Апрувит ненативную монету дл взаимодействия со смарт контрактом router
    async fn approve(&self) {
        info!("Started approving");
        let condition = self
            .approve_contract
            .approve(self.contract_for_approve, U256::max_value())
            .await
            .unwrap();
        match condition {
            true => {
                info!("Contract already approved")
            }
            false => {
                let result = self
                    .approve_contract
                    .approve(self.contract_for_approve, U256::max_value())
                    .send()
                    .await
                    .unwrap()
                    .tx_hash();
                let url = self.url_from.clone() + result.encode_hex().as_str();
                debug!("Result approve:{} for addr:{}", url, self.addr);
            }
        }
    }
    //Дает стоимость транзакции по кроссчейн переводу
    async fn get_fee(&self) -> (U256, LzTxObj) {
        let clouser = |addr: Address| {
            let addres = Bytes::from(addr.0);

            let lz_tx_obj = LzTxObj {
                dst_gas_for_call: U256::from(0),
                dst_native_addr: addres.clone(),
                dst_native_amount: U256::from(0),
            };
            async move {
                let result = self
                    .router_contract
                    .quote_layer_zero_fee(
                        self.chain_to_stargate,
                        1,
                        addres.clone(),
                        Bytes::from_static(b"0x"),
                        lz_tx_obj.clone(),
                    )
                    .await
                    .unwrap()
                    .0;
                info!(
                    "Stargate fee_transaction for {} = {} wei",
                    self.addr, result
                );
                (result, lz_tx_obj)
            }
        };

        if self.currency.as_str() == "USDT" {
            let addr = "0x0000000000000000000000000000000000000001"
                .parse::<Address>()
                .unwrap();
            return clouser(addr).await;
        }
        clouser(self.addr).await
    }
    //Дает баланс ненативной монеты в сети назначения
    async fn get_balance_currency_receiver_chain(&self) -> f64 {
        let balance = self
            .get_balance_contract
            .balance_of(self.addr)
            .await
            .unwrap();
        let balance = format_units(balance, 8).unwrap().parse::<f64>().unwrap();
        info!(
            "Balance in chain destination for {} = {} {}",
            self.addr, balance, &self.currency
        );
        balance
    }
    //Дает баланс нативной монеты в сети отправления
    async fn get_balance_native(&self) -> U256 {
        let balance = self.middleware.get_balance(self.addr, None).await.unwrap();

        info!(
            "Balance Native  currency in sender chain for {} = {} wei",
            self.addr, balance
        );
        balance
    }
    //Дает стоимость нативной монеты сети в usd
    async fn get_price_native_currency_in_usd(&self) -> f64 {
        let price = self.contract.latest_answer().await.unwrap();
        let price = format_units(price, 8).unwrap().parse::<f64>().unwrap();
        info!("Price Native currency {}$", price);
        price
    }
    //Дает gas_price и  выводи в info примерную цену базовой транзакции
    async fn get_gas_price_in_chain(&self) -> U256 {
        let gas_price = self.middleware.get_gas_price().await.unwrap();
        let price_native_currency_in_usd = self.get_price_native_currency_in_usd().await;
        let gas_in_eth = format_units(gas_price * 21000, "eth")
            .unwrap()
            .parse::<f64>()
            .unwrap();

        let gas_in_usd = gas_in_eth.mul(price_native_currency_in_usd);
        info!("Estimate gas_price = {}$", gas_in_usd);
        gas_price
    }
    async fn get_balance_currency_sender_chain(&self) -> U256 {
        let balance = self.approve_contract.balance_of(self.addr).await.unwrap();

        info!(
            "balance currency_to_change={}",
            format_units(balance, 18).unwrap().parse::<f64>().unwrap()
        );

        balance
    }

    async fn dry_run(&self, conract: TypeContract<M>) -> Option<FunctionCall<Arc<M>, M, ()>> {
        let to = Bytes::from_iter(self.addr.0);
        let balance = self.get_balance_currency_sender_chain().await;
        let gas_price = self.get_gas_price_in_chain().await;
        //Конверт цены нативной монеты в долл
        let price_usd_native = self.get_price_native_currency_in_usd().await;
        //Раскидка по монетам для свапа
        match conract {
            TypeContract::OFC20(contract) => {
                let address = "0x0000000000000000000000000000000000000000"
                    .parse::<Address>()
                    .unwrap();
                //Комиссия для свап от Stargate
                let fee_stargate: (U256, U256) = contract
                    .estimate_send_tokens_fee(self.chain_to_stargate, false, Bytes::new())
                    .await
                    .unwrap();
                //Clouser для повторного опроса пройдет ли транза для fee_stargate, которое увеличено
                let clouser = move |uppgrade: usize, functional_call: ContractCall<M, ()>| {
                    let fee = fee_stargate.0.mul(uppgrade).div(100);
                    let fee_in_usd = format_units(fee, "eth").unwrap().parse::<f64>().unwrap()
                        * price_usd_native;
                    warn!("Swap will cost more on {}% == {}$(the contract incorrectly estimated the cost of the swap)",uppgrade-100,fee_in_usd);
                    async move {
                        tokio::time::sleep(Duration::from_secs_f64(0.5)).await;
                        functional_call.value(fee).gas(300000).call().await
                    }
                };
                //Dry_contract для ойенки пройдет ли транза или нет
                let dry_run = contract
                    .send_tokens(
                        self.chain_to_stargate,
                        to.clone(),
                        balance,
                        address,
                        Bytes::new(),
                    )
                    .value(fee_stargate.0)
                    .gas(300000)
                    .gas_price(gas_price);
                match dry_run.call().await {
                    Ok(_succsess) => {
                        let fee_in_usd = format_units(fee_stargate.0, "eth")
                            .unwrap()
                            .parse::<f64>()
                            .unwrap()
                            * price_usd_native;
                        warn!("Swap will cost {}$", fee_in_usd);

                        return Some(dry_run);
                    }
                    Err(_e) => {
                        let mut uppgrade = 110;
                        let mut counter = 1;
                        while let Err(_e) = clouser(uppgrade, dry_run.clone()).await {
                            warn!("{} iteration to determine the Stargate commission", counter);
                            counter += 1;
                            uppgrade += 10;
                        }
                        let fee = fee_stargate.0.mul(uppgrade - 5).div(100);
                        return Some(dry_run.value(fee));
                    }
                }
            }
            TypeContract::ROUTER(contract) => {
                let fee = self.get_fee().await;
                let clouser = move |uppgrade: usize, functional_call: ContractCall<M, ()>| {
                    let fee = fee.0.mul(uppgrade).div(100);
                    let fee_in_usd = format_units(fee, "eth").unwrap().parse::<f64>().unwrap()
                        * price_usd_native;
                    warn!("Swap will cost more on {}% == {}$(the contract incorrectly estimated the cost of the swap)",uppgrade-100,fee_in_usd);
                    async move {
                        tokio::time::sleep(Duration::from_secs_f64(0.5)).await;
                        functional_call.value(fee).gas(400000).call().await
                    }
                };
                let dry_run = contract
                    .swap(
                        self.chain_to_stargate,
                        U256::from(2),
                        U256::from(2),
                        self.addr,
                        balance,
                        balance,
                        fee.1,
                        to,
                        Bytes::from_static(b"0x"),
                    )
                    .value(fee.0)
                    .gas(600000)
                    .gas_price(gas_price);
                match dry_run.call().await {
                    Ok(_s) => Some(dry_run),
                    Err(e) => {
                        let b = e.as_revert().unwrap().as_ref();
                        let v = String::from_utf8_lossy(b);
                        println!("{}", v);

                        let mut counter = 1;
                        let mut uppgrade = 110;

                        while let Err(_e) = clouser(uppgrade, dry_run.clone()).await {
                            warn!("{} iteration to determine the Stargate commission", counter);
                            counter += 1;
                            uppgrade += 10;
                        }
                        let fee = fee.0.mul(uppgrade - 5).div(100);
                        return Some(dry_run.value(fee));
                    }
                }
            }
            _ => None,
        }
    }
    async fn swap(&self) {
        info!("Starting swap for {}", self.addr);
        //Аппруваем контракт
        self.approve().await;
        //"Инициализация контракта"
        match self.currency.as_str() {
            "STG" => {
                let contract = TypeContract::OFC20(self.approve_contract.clone());
                let result_tx = self.dry_run(contract).await.unwrap();
                //Отправка транзы
                let tx = result_tx
                    .send()
                    .await
                    .unwrap()
                    .await
                    .unwrap()
                    .unwrap()
                    .transaction_hash;
                let url = self.url_from.clone() + &tx.to_string();
                println!("Wallet :{},tx:{}", self.addr, url);
                info!("Check carrency in destination chain");
                //Чекаем балик в сети сети назначения
                while self.get_balance_currency_receiver_chain().await == 0.0 {
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
                println!(
                    "Swap will be sucsess.Balance in chain destination:{} ",
                    self.get_balance_currency_receiver_chain().await
                );
            }
            "USDT" => {
                let contract = TypeContract::ROUTER(self.router_contract.clone());
                let result_tx = self.dry_run(contract).await.unwrap();
                //Отправка транзы
                let tx = result_tx
                    .send()
                    .await
                    .unwrap()
                    .await
                    .unwrap()
                    .unwrap()
                    .transaction_hash;
                let url = self.url_from.clone() + &tx.to_string();
                println!("Wallet :{},tx:{}", self.addr, url);
                while self.get_balance_currency_receiver_chain().await == 0.0 {
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
                println!(
                    "Swap will be sucsess.Balance in chain destination:{} ",
                    self.get_balance_currency_receiver_chain().await
                );
            }
            _ => {}
        }
    }
}
