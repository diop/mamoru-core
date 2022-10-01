use crate::value::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait BlockchainDataType: Serialize {
    fn block_hash(&self) -> &String;
    fn block_index(&self) -> u64;
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    block_hash: String,
    block_index: u64,
    transaction_hash: String,
    payload: HashMap<String, Value>,
}

impl BlockchainDataType for Transaction {
    fn block_hash(&self) -> &String {
        &self.block_hash
    }

    fn block_index(&self) -> u64 {
        self.block_index
    }
}

impl Transaction {
    pub fn new(
        block_hash: String,
        block_index: u64,
        transaction_hash: String,
        payload: HashMap<String, Value>,
    ) -> Self {
        Transaction {
            block_hash,
            block_index,
            transaction_hash,
            payload,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    block_hash: String,
    block_index: u64,
    transaction_hash: String,
    event_id: u64,
    payload: HashMap<String, Value>,
}

impl BlockchainDataType for Event {
    fn block_hash(&self) -> &String {
        &self.block_hash
    }

    fn block_index(&self) -> u64 {
        self.block_index
    }
}

impl Event {
    pub fn new(
        block_hash: String,
        block_index: u64,
        transaction_hash: String,
        event_id: u64,
        payload: HashMap<String, Value>,
    ) -> Self {
        Event {
            block_hash,
            block_index,
            transaction_hash,
            event_id,
            payload,
        }
    }
}
