use crate::{errors::RetrieveValueError, value::Value};
use ethnum::U256;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait BlockchainDataType: Serialize {
    fn block_index(&self) -> &Value;
    fn extra(&self) -> &Value;
    fn events(&self) -> Result<&Vec<Event>, RetrieveValueError>;
    fn calltraces(&self) -> Result<&Vec<CallTrace>, RetrieveValueError>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    block_index: Value,
    time: Value,
    transactions: Vec<Transaction>,
    extra: Value,
}

impl BlockchainDataType for Block {
    fn block_index(&self) -> &Value {
        &self.block_index
    }

    fn extra(&self) -> &Value {
        &self.extra
    }

    fn events(&self) -> Result<&Vec<Event>, RetrieveValueError> {
        Err(RetrieveValueError::NoEventsInObject)
    }

    fn calltraces(&self) -> Result<&Vec<CallTrace>, RetrieveValueError> {
        Err(RetrieveValueError::NoCalltracesInObject)
    }
}

impl Block {
    pub fn new(
        block_index: u128,
        time: u64,
        transactions: Vec<Transaction>,
        extra: HashMap<String, Value>,
    ) -> Self {
        Block {
            block_index: Value::UInt128(U256::from(block_index)),
            time: Value::UInt64(U256::from(time)),
            transactions,
            extra: Value::Object(extra),
        }
    }

    pub fn time(&self) -> &Value {
        &self.time
    }

    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    block_index: Value,
    tx_index: Value,
    time: Value,
    events: Vec<Event>,
    calltraces: Vec<CallTrace>,
    extra: Value,
}

impl BlockchainDataType for Transaction {
    fn block_index(&self) -> &Value {
        &self.block_index
    }

    fn extra(&self) -> &Value {
        &self.extra
    }

    fn events(&self) -> Result<&Vec<Event>, RetrieveValueError> {
        Ok(&self.events)
    }

    fn calltraces(&self) -> Result<&Vec<CallTrace>, RetrieveValueError> {
        Ok(&self.calltraces)
    }
}

impl Transaction {
    pub fn new(
        block_index: u128,
        tx_index: u128,
        time: u64,
        events: Vec<Event>,
        calltraces: Vec<CallTrace>,
        extra: HashMap<String, Value>,
    ) -> Self {
        Transaction {
            block_index: Value::UInt128(U256::from(block_index)),
            tx_index: Value::UInt128(U256::from(tx_index)),
            time: Value::UInt64(U256::from(time)),
            events,
            calltraces,
            extra: Value::Object(extra),
        }
    }

    pub fn tx_index(&self) -> &Value {
        &self.tx_index
    }

    pub fn time(&self) -> &Value {
        &self.time
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    block_index: Value,
    tx_index: Value,
    event_index: Value,
    event_id: Value,
    extra: Value,
}

impl BlockchainDataType for Event {
    fn block_index(&self) -> &Value {
        &self.block_index
    }

    fn extra(&self) -> &Value {
        &self.extra
    }

    fn events(&self) -> Result<&Vec<Event>, RetrieveValueError> {
        Err(RetrieveValueError::NoEventsInObject)
    }

    fn calltraces(&self) -> Result<&Vec<CallTrace>, RetrieveValueError> {
        Err(RetrieveValueError::NoCalltracesInObject)
    }
}

impl Event {
    pub fn new(
        block_index: u128,
        tx_index: u128,
        event_index: u128,
        event_id: Vec<u8>,
        extra: HashMap<String, Value>,
    ) -> Self {
        Event {
            block_index: Value::UInt128(U256::from(block_index)),
            tx_index: Value::UInt128(U256::from(tx_index)),
            event_index: Value::UInt128(U256::from(event_index)),
            event_id: Value::Binary(event_id),
            extra: Value::Object(extra),
        }
    }

    pub fn tx_index(&self) -> &Value {
        &self.tx_index
    }

    pub fn event_index(&self) -> &Value {
        &self.event_index
    }

    pub fn event_id(&self) -> &Value {
        &self.event_id
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CallTrace {
    block_index: Value,
    tx_index: Value,
    calltrace_index: Value,
    events: Vec<Event>,
    extra: Value,
}

impl BlockchainDataType for CallTrace {
    fn block_index(&self) -> &Value {
        &self.block_index
    }

    fn extra(&self) -> &Value {
        &self.extra
    }

    fn events(&self) -> Result<&Vec<Event>, RetrieveValueError> {
        Ok(&self.events)
    }

    fn calltraces(&self) -> Result<&Vec<CallTrace>, RetrieveValueError> {
        Err(RetrieveValueError::NoCalltracesInObject)
    }
}

impl CallTrace {
    pub fn new(
        block_index: u128,
        tx_index: u128,
        calltrace_index: u128,
        events: Vec<Event>,
        extra: HashMap<String, Value>,
    ) -> Self {
        CallTrace {
            block_index: Value::UInt128(U256::from(block_index)),
            tx_index: Value::UInt128(U256::from(tx_index)),
            calltrace_index: Value::UInt128(U256::from(calltrace_index)),
            events,
            extra: Value::Object(extra),
        }
    }

    pub fn tx_index(&self) -> &Value {
        &self.tx_index
    }

    pub fn calltrace_index(&self) -> &Value {
        &self.calltrace_index
    }
}

