pub mod string;
pub mod errors;
pub mod list;
pub mod error;
pub mod dbutil;

use string::ValueString;
use errors::OperationError;
use std::collections::{HashMap};


pub enum Value {
    Nil,
    String(ValueString),
    List(ValueList),
    Set(ValueSet),
    SortedSet(ValueSortedSet),
}

#[derive(PartialEq, Debug)]
pub enum PubsubEvent {
    Subscription(Vec<u8>, usize),
    Unsubscription(Vec<u8>, usize),
    PatternSubscription(Vec<u8>, usize),
    PatternUnSubscription(Vec<u8>, usize),
    Message(Vec<u8>, Option<Vec<u8>>, Vec<u8>)
}

impl Value {
    pub fn is_nil(&self) -> bool {
        match *self {
            Value::Nil => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match *self {
            Value::String() => true,
            _ => false,
        }
    }

    pub fn is_set(&self) -> bool {
        match *self {
            Value::Set() => true,
            _ => false,
        }
    }

    pub fn set(&mut self, newvalue: Vec<u8>) -> Result<(), OperationError> {
        match *self {
            Value::String() => Value::String(newvalue.to_string()),
        }

    }

    pub fn get(&self) ->Result<Vec<u8>, OperationError> {

    }

    pub fn strlen(&self) -> Result<usize, OperationError> {

    }

    pub fn append(&mut self, newvalue: Vec<u8>) -> Result<usize, OperationError> {

    }

    pub fn incr(&mut self, incr: i64) -> Result<i64, OperationError> {

    }

    pub fn incrbyfloat(&mut self, incr: f64) -> Result<i64, OperationError> {

    }

    pub fn getrange(&self, start: i64, stop: i64) -> Result<Vec<u8>, OperationError> {

    }

    pub fn setrange(&mut self, index: usize, data: Vec<u8>) -> Result<usize, OperationError> {

    }

    pub fn setbit(&mut self, bitoffset: usize, on: bool) -> Result<bool, OperationError> {

    }

    pub fn getbit(&self, bitoffset: usize) -> Result<bool, OperationError> {

    }

    pub fn pfadd(&mut self, data: Vec<Vec<u8>>) -> Result<bool, OperationError> {

    }

    pub fn pfcount(&self) -> Result<usize, OperationError> {

    }

    pub fn pfmerge(&mut self, values: Vec<&Value>) -> Result<(), OperationError> {

    }

    pub fn push(&mut self, el: Vec<u8>, right: bool) -> Result<usize, OperationError> {

    }

    pub fn pop(&mut self, right: bool) -> Result<Option<Vec<u8>>, OperationError> {

    }

    pub fn lindex(&self, index: i64) -> Result<Option<&[u8]>, OperationError> {

    }

    pub fn linsert(
        &mut self,
        before: bool,
        pivot: Vec<u8>,
        newvalue: Vec<u8>,
    ) -> Result<Option<usize>, OperationError> {

    }

    pub fn llen(&self) -> Result<usize, OperationError> {

    }

    pub fn lrange(&self, start: i64, stop: i64) -> Result<Vec<&[u8]>, OperationError> {

    }

    pub fn lrem(
        &mut self, 
        left: bool,
        limit: usize,
        newvalue: Vec<u8>,
    ) -> Result<usize, OperationError> {

    }

    pub fn lset(&mut self, index: i64, newvalue: Vec<u8>) -> Result<(), OperationError> {

    }

    pub fn ltrim(&mut self, start: i64, end: i64) -> Result<(), OperationError> {

    }


}

pub struct Database {
    pub config: Config,
    
    data: Vec<RehashingHashMap<Vec<u8>, Value>>,
    data_expiration_ms: Vec<RehashingHashMap<Vec<u8>, i64>>,
    watched_keys: Vec<HashMap<Vec<u8>, HashSet<usize>>>,
    subcribers: HashMap<Vec<u8>, SenderMap<Option<Response>>>,
    pattern_subcribers: HashMap<Vec<u8>, SenderMap<Option<Response>>>,
    key_subscribers: Vec<RehashingHashMap<Vec<u8>, SenderMap<bool>>>,
    subscriber_id: usize,
    active_expire_cycle_db: usize,
    monitor_senders: Vec<Sender<String>>,
    pub git_sha1: &'static str,
    pub git_dirty: bool,
    pub version: &'static str,
    pub rustc_version: &'static str,
    pub run_id: String,
    pub start_mstime: i64,
    pub aof: Option<Aof>,
    pub loading: bool,
}

impl Database {
    pub fn new(config: Config) -> Self {

    }

    pub fn uptime(&self) -> i64 {

    }

    fn is_expired (&self, index: usize, key: &[u8]) -> bool {

    }

    pub fn dbsize(&self, index: usize) -> usize {

    }

    pub fn db_expire_size(&self, index: usize) -> usize {

    }

    pub fn get(&self, index: usize, key: &[u8]) -> Option<&value> {

    }

    pub fn get_mut(&mut self, index: usize, key: &[u8]) -> Option<&mut Value> {

    }

    pub fn remove(&mut self, index: usize, key: &[u8]) -> Option<Value> {

    }

    pub fn set_msexpiration(&mut self, index:usize, key: Vec<u8>, msexpiration: i64) {

    }

    pub fn get_msexpiration(&mut self, index: usize, key: &[u8]) -> Option<&i64> {

    }

    
}
