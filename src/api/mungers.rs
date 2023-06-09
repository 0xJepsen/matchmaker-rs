use std::{collections::HashMap, error::Error};
use either::Either;
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
    transaction_data.insert("fast".to_string(), Value::Bool(true)); 

    if let Some(options) = options {
        if let Some(max_block_number) = options.max_block_number {
            transaction_data.insert("maxBlockNumber".to_string(), Value::String(format!("{:#x}", max_block_number)));
        }
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
    let body_data: Vec<Value> = params.body.into_iter().map(|item| {
        match item {
            BodyType::Hash(hash) => {
                let mut new_item = HashMap::new();
                new_item.insert("hash".to_string(), Value::String(hash));
                Value::Object(serde_json::Map::from_iter(new_item.into_iter()))
            },
            BodyType::Tx { tx, can_revert } => {
                let mut new_item = HashMap::new();
                new_item.insert("tx".to_string(), Value::String(tx));
                new_item.insert("can_revert".to_string(), Value::Bool(can_revert));
                Value::Object(serde_json::Map::from_iter(new_item.into_iter()))
            },
            BodyType::Bundle(bundle_params) => {
                let munged_bundle = munge_bundle_params(bundle_params);
                Value::Object(munged_bundle.into_iter().collect::<serde_json::Map<String, Value>>())
            },
        }
    }).collect();

    new_params.insert("body".to_string(), Value::Array(body_data));
    
    // Add the rest of the params as normal
    new_params.insert("version".to_string(), Value::String(params.version.unwrap_or("v0.1".to_string())));
    // Assuming BlockInclusion has block and maxBlock fields.
    new_params.insert("inclusion".to_string(), {
        let mut inclusion = HashMap::new();
        inclusion.insert("block".to_string(), Value::String(format!("{:#x}", params.inclusion.block)));
        if let Some(max_block) = params.inclusion.max_block {
            inclusion.insert("maxBlock".to_string(), Value::String(format!("{:#x}", max_block)));
        }
        Value::Object(inclusion.into_iter().collect())
    });
    // Assuming Validity has refund and refundConfig fields.
    new_params.insert("validity".to_string(), {
        let mut validity = HashMap::new();
        validity.insert("refund".to_string(), Value::Array(
            params.validity.clone()
                .unwrap()
                .refund
                .unwrap_or(Vec::new())
                .into_iter()
                .map(|refund| serde_json::to_value(refund).unwrap())
                .collect()
        ));
        
        validity.insert("refundConfig".to_string(), Value::Array(
            params.validity
                .unwrap()
                .refund_config
                .unwrap_or(Vec::new())
                .into_iter()
                .map(|refund_config| serde_json::to_value(refund_config).unwrap())
                .collect()
        ));
        
        Value::Object(validity.into_iter().collect())
    });
    if let Some(hints) = params.privacy.unwrap().hints {
        let extracted_hints = extract_specified_hints(hints);
        let hints_array: Vec<Value> = extracted_hints.into_iter().map(Value::String).collect();
        new_params.insert("hints".to_string(), Value::Array(hints_array));
    }

    new_params
}



pub fn munge_sim_bundle_options(params: SimBundleOptions) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let mut new_params = HashMap::new();

    match params.parent_block {
        Some(Either::Left(i)) => new_params.insert("parentBlock".to_string(), Value::String(format!("{:#x}", i))),
        Some(Either::Right(s)) => new_params.insert("parentBlock".to_string(), Value::String(s)),
        None => return Err("parentBlock value is missing!".into()),  // Replace with your preferred error handling
    };
    new_params.insert("blockNumber".to_string(), Value::String(format!("{:#x}", params.block_number.unwrap())));
    new_params.insert("timestamp".to_string(), Value::String(format!("{:#x}", params.timestamp.unwrap())));
    new_params.insert("gasLimit".to_string(), Value::String(format!("{:#x}", params.gas_limit.unwrap())));
    new_params.insert("baseFee".to_string(), Value::String(format!("{:#x}", params.base_fee.unwrap())));
    new_params.insert("coinbase".to_string(), Value::String(params.coinbase.unwrap_or(String::new())));
    new_params.insert("timeout".to_string(), Value::String(params.timeout.map(|t| t.to_string()).unwrap_or(String::new())));

    Ok(new_params)
}


