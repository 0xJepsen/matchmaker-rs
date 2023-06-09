use std::collections::HashMap;
use serde_json::Value;
use super::interface::{HintPreferences, BundleParams, TransactionOptions, SimBundleOptions, BodyType};

fn munge_hint_preferences(hints: HintPreferences) -> HashMap<String, bool> {
    let mut munged_hints = HashMap::new();
    munged_hints.insert("contract_address".to_string(), hints.contract_address.unwrap_or(false));
    munged_hints.insert("function_selector".to_string(), hints.function_selector.unwrap_or(false));
    munged_hints.insert("calldata".to_string(), hints.calldata.unwrap_or(false));
    munged_hints.insert("logs".to_string(), hints.logs.unwrap_or(false));
    munged_hints.insert("hash".to_string(), true); // tx hash is always shared on Flashbots Matchmaker; abstract away from user
    // setting all hints except hash to false will enable full privacy
    munged_hints
}

fn extract_specified_hints(hints: HintPreferences) -> Vec<String> {
    let munged_hints = munge_hint_preferences(hints);
    munged_hints.into_iter()
        .filter(|(_, value)| *value)
        .map(|(key, _)| key)
        .collect()
}

pub fn munge_private_tx_params(signed_tx: String, options: Option<TransactionOptions>) -> Vec<HashMap<String, Value>> {
    let mut transaction_data = HashMap::new();
    transaction_data.insert("tx".to_string(), Value::String(signed_tx));
    transaction_data.insert("maxBlockNumber".to_string(), Value::String(options.map(|o| format!("{:#x}", o.max_block_number.unwrap())).unwrap_or(String::new())));
    transaction_data.insert("fast".to_string(), Value::Bool(true)); 

    if let Some(options) = options {
        if let Some(hints) = options.hints {
            let extracted_hints = extract_specified_hints(hints);
            let hints_array: Vec<Value> = extracted_hints.into_iter().map(Value::String).collect();
            transaction_data.insert("hints".to_string(), Value::Array(hints_array));
        }
        if let Some(builders) = options.builders {
            let builders_array: Vec<Value> = builders.into_iter().map(Value::String).collect();
            transaction_data.insert("builders".to_string(), Value::Array(builders_array));
        }
    }

    vec![transaction_data]
}


pub fn munge_bundle_params(params: BundleParams) -> HashMap<String, Value> {
    let mut new_params = HashMap::new();

    // We need to handle the body data more carefully as it is an array of multiple types
    let body_data: Vec<Value> = params.body.iter().map(|item| {
        match item {
            BodyType::Hash(hash) => {
                // Do something with hash
            }
            BodyType::Tx { tx, can_revert } => {
                // Do something with tx and can_revert
            }
            BodyType::Bundle(bundle) => {
                // If you find a bundle, munge it and insert
                let munged_bundle = munge_bundle_params(bundle.clone());
                let munged_bundle_map: serde_json::Map<String, Value> = munged_bundle.into_iter().collect();
                new_params.insert("bundle".to_string(), Value::Object(munged_bundle_map));

            }
        }
    }).collect();

    new_params.insert("body".to_string(), Value::Array(body_data));
    new_params.insert("version".to_string(), Value::String(params.version.unwrap_or("v0.1".to_string())));
    new_params.insert("block".to_string(), Value::String(format!("{:#x}", params.inclusion.block)));
    if let Some(max_block) = params.inclusion.max_block {
        new_params.insert("maxBlock".to_string(), Value::String(format!("{:#x}", max_block)));
    }

    // Update how you handle validity
    let validity_value = match params.validity {
        Some(validity) => {
            let mut validity_map = HashMap::new();
            validity_map.insert("refund".to_string(), Value::Array(validity.refund.iter().map(|x| Value::String(x.clone())).collect()));
            validity_map.insert("refundConfig".to_string(), Value::Array(validity.refund_config.iter().map(|x| Value::String(x.clone())).collect()));
            Value::Object(validity_map)
        }
        None => {
            let mut default_validity = HashMap::new();
            default_validity.insert("refund".to_string(), Value::Array(Vec::new()));
            default_validity.insert("refundConfig".to_string(), Value::Array(Vec::new()));
            Value::Object(default_validity)
        }
    };

    new_params.insert("validity".to_string(), validity_value);

    if let Some(privacy) = params.privacy {
        if let Some(hints) = privacy.hints {
            let extracted_hints = extract_specified_hints(hints);
            let hints_array: Vec<Value> = extracted_hints.into_iter().map(Value::String).collect();
            new_params.insert("hints".to_string(), Value::Array(hints_array));
        }
    }
    
    new_params
}


pub fn munge_sim_bundle_options(params: SimBundleOptions) -> HashMap<String, Value> {
    let mut new_params = HashMap::new();

    new_params.insert("parentBlock".to_string(), Value::String(format!("{:#x}", params.parent_block.unwrap())));
    new_params.insert("blockNumber".to_string(), Value::String(format!("{:#x}", params.block_number.unwrap())));
    new_params.insert("timestamp".to_string(), Value::String(format!("{:#x}", params.timestamp.unwrap())));
    new_params.insert("gasLimit".to_string(), Value::String(format!("{:#x}", params.gas_limit.unwrap())));
    new_params.insert("baseFee".to_string(), Value::String(format!("{:#x}", params.base_fee.unwrap())));
    new_params.insert("coinbase".to_string(), Value::String(params.coinbase.unwrap_or(String::new())));
    new_params.insert("timeout".to_string(), Value::String(params.timeout.map(|t| t.to_string()).unwrap_or(String::new())));

    new_params
}


