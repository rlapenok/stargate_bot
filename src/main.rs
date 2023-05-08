use helpers_fn::read_curr_from_to;

use crate::helpers_fn::from_private_key_config_proxy::from_private_key_config_proxy;

pub mod helpers_fn;
pub mod user_wallet_eth;
pub mod traits;
fn main() {
    //Будет подтягивать данные из .env
    dotenv::dotenv().unwrap();
    //Будет писать логи 
    env_logger::init();

    let v=read_curr_from_to::read_curr_from_to();
    println!("{:?}",v);

    let b=from_private_key_config_proxy(v);
}
