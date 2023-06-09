use crate::api::{interface::MatchmakerNetwork, networks::SupportedNetworks};

pub struct Matchmaker<D> 
where 
    D: ethers::providers::Middleware 
    + ethers::core::k256::ecdsa::signature::hazmat::PrehashSigner<
        (
            ethers::core::k256::ecdsa::Signature,
            ethers::core::k256::ecdsa::RecoveryId
        )
    >
{
    auth_signer: ethers::signers::Wallet<D>,
    network: MatchmakerNetwork,
}


impl<D> Matchmaker<D> 
where 
    D: ethers::providers::Middleware 
    + ethers::core::k256::ecdsa::signature::hazmat::PrehashSigner<
        (
            ethers::core::k256::ecdsa::Signature,
            ethers::core::k256::ecdsa::RecoveryId
        )
    >
{
    pub fn use_ethereum_mainnet(auth_signer: ethers::signers::Wallet<D>) -> Self {
        let network = SupportedNetworks::mainnet();
        Matchmaker {
            auth_signer,
            network: MatchmakerNetwork {
                chain_id: network.chain_id,
                name: network.name.to_string(),
                stream_url: network.stream_url.to_string(),
                api_url: network.api_url.to_string(),
            },
        }
        
    }

    pub fn use_ethereum_goerli(auth_signer: ethers::signers::Wallet<D>) -> Self {
        let network = SupportedNetworks::goerli();
        Matchmaker {
            auth_signer,
            network: MatchmakerNetwork {
                chain_id: network.chain_id,
                name: network.name.to_string(),
                stream_url: network.stream_url.to_string(),
                api_url: network.api_url.to_string(),
            },
        }
    }
}
