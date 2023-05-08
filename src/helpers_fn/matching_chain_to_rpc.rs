use ethers::types::Chain;

pub fn matching_chain_to_rpc(chain:&String)->Option<String>{

    match chain.as_str() {
            "BNB"=>{
                Some(dotenv::var("BNB_RPC").unwrap())
            }
            "AVAX"=>{
                Some(dotenv::var("AVAX_RPC").unwrap())

            }
            "POL"=>{
                Some(dotenv::var("POL_RPC").unwrap())

            }
            "ARB"=>{
                Some(dotenv::var("ARB_RPC").unwrap())

            }
            "OPT"=>{
                Some(dotenv::var("OPT_RPC").unwrap())

            }
            "FANTOM"=>{
                Some(dotenv::var("FANTOM_RPC").unwrap())

            }
            _=>{
                None
            }    
    }


}