use helpers_fn::read_curr_from_to;

use crate::helpers_fn::from_private_key_config_proxy::from_config_to_user_wallets;

pub mod helpers_fn;
pub mod user_wallet_eth;
pub mod traits;
pub mod contractss;
fn main() {
    //Будет подтягивать данные из .env
    dotenv::dotenv().unwrap();
    //Будет писать логи 
    env_logger::init();

}
