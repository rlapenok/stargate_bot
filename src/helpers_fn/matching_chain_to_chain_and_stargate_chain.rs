use ethers::types::Chain;
use log::warn;


pub fn matching_chain_to_chain_and_stargate_chain(chains:&[String])->(Option<Chain>,Option<u16>){

    let (chain_from,chain_to_stargate);
    match chains[0].clone().as_str() {
        "BNB"=>{
            chain_from=Some(Chain::BinanceSmartChain);
              
        }
        "AVAX"=>{
            chain_from=Some(Chain::Avalanche);

        }
        "POL"=>{
            chain_from=Some(Chain::Polygon);

        }
        "ARB"=>{
            chain_from=Some(Chain::Arbitrum);

        }
        "OPT"=>{
            chain_from=Some(Chain::Optimism);

        }
        "FANTOM"=>{
            chain_from=Some(Chain::Fantom);

        }
        _=>{
            chain_from=None;
            warn!(" Chain {} not supported ", chains[0].as_str());
        }    
}
match chains[1].clone().as_str() {
    "BNB"=>{
        chain_to_stargate=Some(102 as u16);
          
    }
    "AVAX"=>{
        chain_to_stargate=Some(106 as u16);


    }
    "POL"=>{
        chain_to_stargate=Some(109 as u16);


    }
    "ARB"=>{
        chain_to_stargate=Some(110 as u16);


    }
    "OPT"=>{
        chain_to_stargate=Some(111 as u16);
        

    }
    "FANTOM"=>{
        chain_to_stargate=Some(112 as u16);


    }
    _=>{
        chain_to_stargate=None;
        warn!(" Chain {} not supported ", chains[1].as_str());
    }    
}
    (chain_from,chain_to_stargate)

}