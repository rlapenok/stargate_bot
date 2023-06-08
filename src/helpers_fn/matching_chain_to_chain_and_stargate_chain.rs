use ethers::types::Chain;
use log::warn;

pub fn matching_chain_to_chain_and_stargate_chain(
    chains: &[String],
) -> (
    (Option<Chain>, Option<u16>),
    (Option<String>, Option<String>),
) {
    let (url_check_transt_chain_from, url_check_trans_chain_to);
    let (chain_from, chain_to_stargate);
    match chains[0].clone().as_str() {
        "BNB" => {
            chain_from = Some(Chain::BinanceSmartChain);
            url_check_transt_chain_from = Some(String::from("https://bscscan.com/tx/"));
        }
        "AVAX" => {
            chain_from = Some(Chain::Avalanche);
            url_check_transt_chain_from =
                Some(String::from("https://avascan.info/blockchain/c/tx/"));
        }
        "POL" => {
            chain_from = Some(Chain::Polygon);
            url_check_transt_chain_from = Some(String::from("https://polygonscan.com/tx/"));
        }
        "ARB" => {
            chain_from = Some(Chain::Arbitrum);
            url_check_transt_chain_from = Some(String::from("https://arbiscan.io/tx/"));
        }
        "OPT" => {
            chain_from = Some(Chain::Optimism);
            url_check_transt_chain_from = Some(String::from("https://optimistic.etherscan.io/tx/"));
        }
        "FANTOM" => {
            chain_from = Some(Chain::Fantom);
            url_check_transt_chain_from = Some(String::from("https://ftmscan.com/tx/"));
        }
        _ => {
            chain_from = None;
            url_check_transt_chain_from = None;
            warn!(" Chain {} not supported ", chains[0].as_str());
        }
    }
    match chains[1].clone().as_str() {
        "BNB" => {
            chain_to_stargate = Some(102 as u16);
            url_check_trans_chain_to = Some(String::from("https://bscscan.com/tx/"));
        }
        "AVAX" => {
            chain_to_stargate = Some(106 as u16);
            url_check_trans_chain_to = Some(String::from("https://avascan.info/blockchain/c/tx/"));
        }
        "POL" => {
            chain_to_stargate = Some(109 as u16);
            url_check_trans_chain_to = Some(String::from("https://polygonscan.com/tx/"));
        }
        "ARB" => {
            chain_to_stargate = Some(110 as u16);
            url_check_trans_chain_to = Some(String::from("https://arbiscan.io/tx/"));
        }
        "OPT" => {
            chain_to_stargate = Some(111 as u16);
            url_check_trans_chain_to = Some(String::from("https://optimistic.etherscan.io/tx/"));
        }
        "FANTOM" => {
            chain_to_stargate = Some(112 as u16);
            url_check_trans_chain_to = Some(String::from("https://ftmscan.com/tx/"));
        }
        _ => {
            chain_to_stargate = None;
            url_check_trans_chain_to = None;
            warn!(" Chain {} not supported ", chains[1].as_str());
        }
    }
    (
        (chain_from, chain_to_stargate),
        (url_check_transt_chain_from, url_check_trans_chain_to),
    )
}
