use helpers_fn::read_curr_from_to::read_curr_from_to;
use traits::trait_for_user_wallets::MyMiddleware;

use crate::helpers_fn::from_config_key_config_proxy::from_config_to_user_wallets;

pub mod contractss;
pub mod helpers_fn;
pub mod traits;
pub mod user_wallet_eth;

#[tokio::main]
async fn main() {
    //Будет подтягивать данные из .env
    dotenv::dotenv().unwrap();
    //Будет писать логи
    env_logger::init();

    let x = read_curr_from_to();
    let v = from_config_to_user_wallets(x);
    let b = &v[0];
    b.swap().await;
}
