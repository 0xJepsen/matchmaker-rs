use either::Either;
use ethers::prelude::Log as LogParams;
use serde::Serialize;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamEvent {
    Bundle,
    Transaction,
}
#[derive(Debug, Clone)]
pub struct Tx {
    pub(crate) tx: String,
    pub(crate) can_revert: bool,
}

pub struct MatchmakerNetwork {
    pub(crate) chain_id: i32,
    pub(crate) name: String,
    pub(crate) stream_url: String,
    pub(crate) api_url: String,
}
#[derive(Debug, Clone)]

pub struct HintPreferences {
    pub(crate) calldata: Option<bool>,
    pub(crate) contract_address: Option<bool>,
    pub(crate) function_selector: Option<bool>,
    pub(crate) logs: Option<bool>,
}

pub struct TransactionOptions {
    pub(crate) hints: Option<HintPreferences>,
    pub(crate) max_block_number: Option<i32>,
    pub(crate) builders: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Inclusion {
    pub(crate) block: i32,
    pub(crate) max_block: Option<i32>,
}

#[derive(Debug, Clone)]

pub enum BodyType {
    Hash(String),
    Tx { tx: String, can_revert: bool },
    Bundle(BundleParams),
}

#[derive(Debug, Clone)]
pub struct BundleParams {
    pub(crate) version: Option<String>,
    pub(crate) inclusion: Inclusion,
    pub(crate) body: Vec<BodyType>,
    pub(crate) validity: Option<Validity>,
    pub(crate) privacy: Option<Privacy>,
    pub(crate) metadata: Option<Metadata>,
}
#[derive(Debug, Clone, Serialize, Copy)]
pub struct Refund {
    body_idx: i32,
    percent: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct RefundConfig {
    address: String,
    percent: i32,
}
#[derive(Debug, Clone)]

pub struct Validity {
    pub(crate) refund: Option<Vec<Refund>>,
    pub(crate) refund_config: Option<Vec<RefundConfig>>,
}
#[derive(Debug, Clone)]

pub struct Privacy {
    pub(crate) hints: Option<HintPreferences>,
    pub(crate) builders: Option<Vec<String>>,
}
#[derive(Debug, Clone)]

pub struct Metadata {
    origin_id: Option<String>,
}

pub struct ISendBundleResponse {
    bundle_hash: String,
}

pub struct ISendBundleResult {
    bundle_hash: String,
}

impl ISendBundleResult {
    pub fn new(response: ISendBundleResponse) -> Self {
        Self {
            bundle_hash: response.bundle_hash,
        }
    }
}

pub struct SimBundleOptions {
    pub(crate) parent_block: Option<Either<i32, String>>,
    pub(crate) block_number: Option<i32>,
    pub(crate) coinbase: Option<String>,
    pub(crate) timestamp: Option<i64>,
    pub(crate) gas_limit: Option<i32>,
    pub(crate) base_fee: Option<u128>,
    pub(crate) timeout: Option<i32>,
}

pub struct SimBundleLogs {
    tx_logs: Option<Vec<LogParams>>,
    bundle_logs: Option<Vec<SimBundleLogs>>,
}

pub struct ISimBundleResponse {
    success: bool,
    error: Option<String>,
    state_block: String,
    mev_gas_price: String,
    profit: String,
    refundable_value: String,
    gas_used: String,
    logs: Option<Vec<SimBundleLogs>>,
}

pub struct ISimBundleResult {
    success: bool,
    error: Option<String>,
    state_block: u32,
    mev_gas_price: u128,
    profit: u128,
    refundable_value: u128,
    gas_used: u128,
    logs: Option<Vec<SimBundleLogs>>,
}

impl ISimBundleResult {
    pub fn new(response: ISimBundleResponse) -> Self {
        Self {
            success: response.success,
            error: response.error,
            state_block: u32::from_str_radix(&response.state_block[2..], 16).unwrap_or(0),
            mev_gas_price: u128::from_str_radix(&response.mev_gas_price[2..], 16).unwrap_or(0),
            profit: u128::from_str_radix(&response.profit[2..], 16).unwrap_or(0),
            refundable_value: u128::from_str_radix(&response.refundable_value[2..], 16).unwrap_or(0),
            gas_used: u128::from_str_radix(&response.gas_used[2..], 16).unwrap_or(0),
            logs: response.logs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxData {
    pub(crate) to: String,
    pub(crate) function_selector: String,
    pub(crate) call_data: String,
}

pub struct IMatchmakerEvent {
    pub hash: String, // Transaction or Bundle hash.
    pub logs: Option<Vec<LogParams>>, // Logs emitted by the transaction or bundle.
    pub txs: Option<Vec<TxData>>, // Transaction data.
    pub mev_gas_price: Option<String>, // Change in coinbase value after inserting tx/bundle, divided by gas used.
    pub gas_used: Option<String>, // Gas used by the tx/bundle, rounded up to 2 most significant digits.
}


pub struct IPendingTransaction {
    pub(crate) hash: String,
    pub(crate) logs: Option<Vec<LogParams>>,
    pub(crate) to: Option<String>,
    pub(crate) function_selector: Option<String>,
    pub(crate) call_data: Option<String>,
    pub(crate) mev_gas_price: Option<u128>,
    pub(crate) gas_used: Option<u128>,
}

pub struct IPendingBundle {
    hash: String,
    logs: Option<Vec<LogParams>>,
    txs: Option<Vec<TxData>>,
    mev_gas_price: Option<u128>,
    gas_used: Option<u128>,
}