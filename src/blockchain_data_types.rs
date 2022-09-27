use crate::value::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait BlockchainDataType: Serialize {
    fn block_hash(&self) -> &String;
    fn block_index(&self) -> usize;
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    block_hash: String,
    block_index: usize,
    transaction_hash: String,
    payload: HashMap<String, Value>,
}

impl BlockchainDataType for Transaction {
    fn block_hash(&self) -> &String {
        &self.block_hash
    }

    fn block_index(&self) -> usize {
        self.block_index
    }
}

impl Transaction {
    pub fn new(
        block_hash: String,
        block_index: usize,
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
    block_index: usize,
    transaction_hash: String,
    event_id: usize,
    payload: HashMap<String, Value>,
}

impl BlockchainDataType for Event {
    fn block_hash(&self) -> &String {
        &self.block_hash
    }

    fn block_index(&self) -> usize {
        self.block_index
    }
}

impl Event {
    pub fn new(
        block_hash: String,
        block_index: usize,
        transaction_hash: String,
        event_id: usize,
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
