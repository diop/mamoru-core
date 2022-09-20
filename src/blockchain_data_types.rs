use serde_json::{Value, Map};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    block_hash: String,
    block_index: usize,
    transaction_hash: String, 
    payload: Map<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    block_hash: String,
    block_index: usize,
    transaction_hash: String,
    event_id: usize,
    payload: Map<String, Value>,
}