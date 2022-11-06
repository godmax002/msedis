use std::{sync::mpsc::Receiver, fmt::Formatter};

pub enum Response {
    Nil,
    Integer(i64),
    Data(Vec<u8>),
    Error(String),
    Status(String),
    Array(Vec<Response>),
}

pub enum ResponseError {
    NoReply,
    Wait(Receiver<Option<OwnedParsedCommand>>),
}


impl Debug for ResponseError {
    fn fmt(&self, f: &mut Formatter) {
    
    }
}

impl Response {
    pub fn as_bytes(&self) -> Vec<u8> {

    }

    pub fn is_error(&self) -> bool {

    }

    pub fn is_status(&self) -> bool {

    }
}