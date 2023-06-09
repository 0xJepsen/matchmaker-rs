use ethers::prelude::Log;

use super::interface::{IMatchmakerEvent, TxData};

pub struct PendingTransaction {
    hash: String,
    logs: Option<Vec<Log>>,
    to: Option<String>,
    function_selector: Option<String>,
    call_data: Option<String>,
    mev_gas_price: Option<String>,
    gas_used: Option<String>,
}

impl PendingTransaction {
    pub fn new(event: &IMatchmakerEvent) -> Self {
        let tx = event.txs.as_ref().and_then(|v| v.get(0));

        Self {
            hash: event.hash.clone(),
            logs: event.logs.clone(),
            to: tx.map(|t| t.to.clone()),
            function_selector: tx.map(|t| t.function_selector.clone()),
            call_data: tx.map(|t| t.call_data.clone()),
            gas_used: event.gas_used,
            mev_gas_price: event.mev_gas_price,
        }
    }
}

pub struct PendingBundle {
    hash: String,
    logs: Option<Vec<Log>>,
    txs: Option<Vec<TxData>>,
    mev_gas_price: Option<String>,
    gas_used: Option<String>,
}

impl PendingBundle {
    pub fn new(event: &IMatchmakerEvent) -> Self {
        Self {
            hash: event.hash.clone(),
            logs: event.logs.clone(),
            txs: event.txs.clone(),
            gas_used: event.gas_used,
            mev_gas_price: event.mev_gas_price,
        }
    }
}
