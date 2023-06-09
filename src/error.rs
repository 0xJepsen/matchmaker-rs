use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatchmakerError {
    #[error("JsonRpcError: {code}: {message}")]
    JsonRpcError { code: i32, message: String },
    
    #[error("NetworkFailure: {status}: {data}, {stack}")]
    NetworkFailure { status: u16, data: String, stack: String },
    
    #[error("Cannot infer network params from chainId: {chain_id}")]
    UnimplementedNetwork { chain_id: i32 },
    
    #[error("Unimplemented stream event type: {event_type}")]
    UnimplementedStreamEvent { event_type: String },
    
    #[error("{0}")]
    Other(String),
}
