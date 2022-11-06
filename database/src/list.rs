use std::collections::LinkedList;

pub enum ValueList {
    Data(LinkedList<Vec<u8>>)
}

impl Default for ValueList {
    fn default() -> Self {
        Self::new();
    }
}

impl ValueList {
    pub fn new() -> Self {
        ValueList::Data(LinkedList::new())
    }

    pub fn push(&mut self, el: Vec<u8>, right: bool) {
        match *self {
            ValueList::Data(ref mut list) => {
                if right {
                    list.push_back(el);
                } else {
                    list.push_front(el);
                }
            }
        }
    }

    pub fn pop(&mut self, right: bool) ->Option<Vec<u8>> {
        match *self {
            ValueList::Data(ref mut list) => {
                if right {
                    list.pop_back()
                } else {
                    list.pop_front()
                }
            }
        }
    }

    pub fn lindex(&mut self, _index: i64) -> Option<&[u8]> {
        match *self {
            ValueList::Data(ref list) => {
                let index = match normalize_position(_index, list.len()) {
                    Ok(i) => i,
                    Err(_) => return None,
                };
                list.iter().nth(index as usize).map(|a| &a[..])
            }
        }
    }

    pub fn linsert(&mut self, before: bool, pivot: Vec<u8>, newvalue: Vec<u8>) -> Option<usize> {
        match *self {
            ValueList::Data(ref mut list) => match list.iter().position(|x| x == &pivot) {
                Some(_pos) => {
                    let pos = if before {_pos} else {_pos + 1};
                    let mut right = list.split_off(pos);
                    list.push_back(newvalue);
                    list.append(&mut right);
                    Some(list.len())
                },
                None => None,
            }
        }
    }

    pub fn llen(&self) -> usize {
        match *self {
            ValueList::Data(ref list) => list.len()
        }
    }

    pub fn lrange(&self, _strt: i64, _stop: i64) -> Vec<&[u8]> {

    }

    pub fn lrem(&mut self, left: bool, limit: usize, newvalue: Vec<u8>) -> usize {

    }

    pub fn lset(&mut self, index: i64, newvalue: Vec<u8>) -> Result<(), OperationError> {

    }

    pub fn ltrim(&mut self, _start: i64, _stop: i64) -> Result<(), OperationError> {

    }

    pub fn dump<T: Write>(&self, writer: &mut T) -> io::Result<usize> {

    }

    pub fn debug_object (&self) -> String {

    }
}