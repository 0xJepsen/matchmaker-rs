use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref SUPPORTED_NETWORKS: HashMap<&'static str, Network> = {
        let mut m = HashMap::new();
        m.insert("mainnet", Network {
            name: "mainnet",
            chain_id: 1,
            stream_url: "https://mev-share.flashbots.net",
            api_url: "https://relay.flashbots.net",
        });
        m.insert("goerli", Network {
            name: "goerli",
            chain_id: 5,
            stream_url: "https://mev-share-goerli.flashbots.net",
            api_url: "https://relay-goerli.flashbots.net",
        });  
        m
    };
}

// Define the Error struct as per your application
#[derive(Debug)]
pub struct UnimplementedNetwork {
    chain_id: i32,
}

#[derive(Debug)]
pub struct Network {
    pub(crate) name: &'static str,
    pub(crate) chain_id: i32,
    pub(crate) stream_url: &'static str,
    pub(crate) api_url: &'static str,
}

pub struct SupportedNetworks;

impl SupportedNetworks {
    fn supported_networks() -> HashMap<&'static str, Network> {
        let mut map = HashMap::new();
        map.insert(
            "mainnet",
            Network {
                name: "mainnet",
                chain_id: 1,
                stream_url: "https://mev-share.flashbots.net",
                api_url: "https://relay.flashbots.net",
            },
        );
        map.insert(
            "goerli",
            Network {
                name: "goerli",
                chain_id: 5,
                stream_url: "https://mev-share-goerli.flashbots.net",
                api_url: "https://relay-goerli.flashbots.net",
            },
        );
        map
    }

    pub fn mainnet() -> &'static Network {
        &SUPPORTED_NETWORKS["mainnet"]
    }

    pub fn goerli() -> &'static Network {
        &SUPPORTED_NETWORKS["goerli"]
    }

    pub fn supports_chain_id(chain_id: i32) -> bool {
        let networks = Self::supported_networks();
        networks.values().any(|n| n.chain_id == chain_id)
    }

    pub fn get_network(chain_id: i32) -> Result<&'static Network, UnimplementedNetwork> {
        SUPPORTED_NETWORKS
            .values()
            .find(|&n| n.chain_id == chain_id)
            .ok_or(UnimplementedNetwork { chain_id })
    }
    
}
