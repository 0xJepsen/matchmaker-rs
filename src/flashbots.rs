use ethers::signers::Signer;
use ethers::core::types::{TransactionRequest, Signature};
use ethers::prelude::LocalWallet;
use serde_json::json;
use std::collections::HashMap;

pub async fn get_rpc_request(params: HashMap<String, String>, method: String, auth_signer: LocalWallet) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn std::error::Error>> {
    let body = json!({
        "params": params,
        "method": method,
        "id": 69,
        "jsonrpc": "2.0"
    });

    let body_string = body.to_string();
    
    // Sign the message
    let signature = auth_signer.sign_message(&body_string).await?;

    // Construct the headers
    let signature_string = format!("{}:{}", auth_signer.address(), signature);
    let headers = json!({
        "Content-Type": "application/json",
        "X-Flashbots-Signature": signature_string
    });

    Ok(json!({
        "headers": headers,
        "signature": signature_string,
        "body": body_string
    }))
}
