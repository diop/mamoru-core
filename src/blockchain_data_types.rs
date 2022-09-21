use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Value {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

pub trait BlockchainDataType : Serialize {
    fn get_block_hash(&self) -> &String;
    fn get_block_index(&self) -> usize;
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    block_hash: String,
    block_index: usize,
    transaction_hash: String, 
    payload: HashMap<String, Value>,
}

impl BlockchainDataType for Transaction {
    fn get_block_hash(&self) -> &String {
        &self.block_hash
    }

    fn get_block_index(&self) -> usize {
        self.block_index
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
    fn get_block_hash(&self) -> &String {
        &self.block_hash
    }

    fn get_block_index(&self) -> usize {
        self.block_index
    }
}