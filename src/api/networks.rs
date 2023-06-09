use std::collections::HashMap;

// Define the Error struct as per your application
#[derive(Debug)]
struct UnimplementedNetwork {
    chain_id: i32,
}

#[derive(Debug)]
struct Network {
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
        &Self::supported_networks()["mainnet"]
    }

    pub fn goerli() -> &'static Network {
        &Self::supported_networks()["goerli"]
    }

    pub fn supports_chain_id(chain_id: i32) -> bool {
        let networks = Self::supported_networks();
        networks.values().any(|n| n.chain_id == chain_id)
    }

    pub fn get_network(chain_id: i32) -> Result<&'static Network, UnimplementedNetwork> {
        let networks = Self::supported_networks();
        networks.values().find(|&n| n.chain_id == chain_id).map(|network| network).ok_or(UnimplementedNetwork { chain_id })
    }
}
