use std::sync::Arc;

use ethers::{prelude::FunctionCall, providers::Middleware, types::U256};

use crate::contractss::{LzTxObj, Router, OFC20};

pub enum TypeContract<M> {
    OFC20(Arc<OFC20<M>>),
    ROUTER(Arc<Router<M>>),
}

pub trait ForUserWallet {
    fn print(&self);
}

#[async_trait::async_trait]
pub trait MyMiddleware<M: Middleware> {
    async fn approve(&self);
    async fn get_fee(&self) -> (U256, LzTxObj);
    async fn get_balance_currency_receiver_chain(&self) -> f64;
    async fn get_balance_native(&self) -> U256;
    async fn get_price_native_currency_in_usd(&self) -> f64;
    async fn get_gas_price_in_chain(&self) -> U256;
    async fn get_balance_currency_sender_chain(&self) -> U256;
    async fn dry_run(&self, contract: TypeContract<M>) -> Option<FunctionCall<Arc<M>, M, ()>>;
    async fn swap(&self);
}
