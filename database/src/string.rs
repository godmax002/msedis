

use std::str;
use std::io;
use std::io::Write;

use crate::errors::OperationError;


#[derive(PartialEq, Debug, Clone)]
pub enum ValueString {
    Integer(i64),
    Data(Vec<u8>),
}

fn parse_utf8<T: str::FromStr>(newvalue: &[u8]) -> Option<T> {
    if let Some(b'0') = newvalue.first() {
        None
    } else {
        // todo: think
        str::from_utf8(newvalue).ok().and_then(|s| s.parse().ok())
    }
}

impl ValueString {
    pub fn new(newvalue: Vec<u8>) -> Self {
        match parse_utf8(&newvalue) {
            Some(i) => ValueString::Integer(i),
            None =>ValueString::Data(newvalue),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        // todo: why not *self
        match self {
            ValueString::Data(data) => data.clone(),
            ValueString::Integer(int) =>int.to_string().into_bytes(),
        }
    }

    pub fn strlen(&self) -> usize {
        match self {
            ValueString::Data(data) =>data.len(),
            ValueString::Integer(int) => int.to_string().len(),
        }

    }
    pub fn append(&mut self, newvalue: Vec<u8>) {
        match self {
            ValueString::Data(data) => data.extend(newvalue),
            ValueString::Integer(i) => {
                let oldstr = i.to_string();
                *self = ValueString::new([oldstr.into_bytes(), newvalue].concat());
            }
        }
    }

    pub fn incr(&mut self, incr: i64) -> Result<i64, OperationError> {
        let val = match self {
            ValueString::Integer(i) => *i,
            ValueString::Data(data) => parse_utf8(data).ok_or_else(|| {
                OperationError::ValueError("ERR value is not a valid integer".to_owned())
            })?,
        };
        let newval = val.checked_add(incr).ok_or(OperationError::OverflowError)?;
        *self = ValueString::Integer(newval);
        Ok(newval)
    }

    pub fn incrbyfloat(&mut self, incr: f64) ->Result<f64, OperationError> {

    }

    pub fn getrange(&self, _start: i64, _stop: i64) -> Vec<u8> {

        let s = match self {
            ValueString::Integer(i) => format!("{}", i).into_bytes(),
            ValueString::Data(s) => s.clone(),
        };

        let len = s.len();
        let start = match normalize_position(_start, len) {
            Ok(i) => i,
            Err(g) => {
                if !g {
                    0
                } else {
                    return Vec::new();
                }
            }
        } as usize;
        let stop = match normalize_position(_stop, len) {
            Ok(i) => i,
            Err(g) => {
                if !g {
                    return Vec::new();
                } else {
                    len - 1
                }
            }
        } as usize;

        if stop < start {
            return Vec::new();
        }

        let mut v = Vec::with_capacity(stop - start + 1);
        v.extend(s[start..stop + 1].iter());
        v

    }

    pub fn setbit(&mut self, bitoffset: usize, on: bool) -> bool {
        match self {
            ValueString::Integer(i) => *self = &mut ValueString::Data(format!("{}", i).into_bytes()),
            ValueString::Data(_) => (),
        };

        let d = match self {
            ValueString::Data(d) => d,
            _ => panic!("value should be data"),
        };

        let byte = bitoffset >> 3;
        // todo think logic
        while byte + 1 > d.len() {
            d.push(0);
        }

        let mut byteval = d[byte];
        let bit = 7 - (bitoffset & 0x7);
        let bitval = byteval & (1 << bit);

        byteval &= !(1 << bit);
        byteval |= (if on {1} else {0} & 0x1) << bit;
        d[byte] = byteval;

        bitval != 0
    }

    pub fn getbit(&self, bitoffset:usize) -> bool {
        let tmp;
        let d = match self {
            ValueString::Integer(i) => {
                tmp = format!("{}", i).into_bytes();
                &tmp
            },
            ValueString::Data(d) => d,
        };

        let byte = bitoffset >> 3;
        if byte >= d.len() {
            return false;
        }

        let bit = 7 - (bitoffset & 0x7);
        let bitval = d[byte] & (1 << bit);

        bitval  != 0
    }

    pub fn setrange(&mut self, _index: usize, data: Vec<u8>) -> usize {
        if data.is_empty() {
            return self.strlen();
        }

        match self {
            ValueString::Integer(i) => *self = &mut ValueString::Data(format!("{}", i).into_bytes()),
            ValueString::Data(_) => (),
        }

        let d = match self {
            ValueString::Data(s) => s,
            _ => panic("String must be data"),
        };

        let mut index = _index;
        for _ in d.len()..index {
            d.push(0);
        }
        for c in data {
            d.push(c);
            if index < d.len() - 1 {
                d.swap_remove(index);
            }
            index  += 1;
        }
        d.len()

    }

    pub fn pfadd(&mut self, data: Vec<Vec<u8>>) -> Result<bool, OperationError> {
        let mut changed = false;
        let mut hll = if self.strlen() == 0 {
            HLL::new(HLL_ERROR)
        }else {
            HLL::from_vec(self.to_vec())
        };

        for el in data {
            changed = hll.insert(&el) || changed;
        }

        if changed {
            *self = ValueString::new(hll.into_vec());
        }
        Ok(changed)
    }

    pub fn pfcount(&self) ->Result<usize, OperationError> {

    }

    pub fn pfmerge(&mut self, data: Vec<&ValueString>) -> Result<(), OperationError> {

    }

    pub fn dump<T: Write>(&self, writer: &mut T) -> io::Result<usize> {
        let mut v = vec![];
        match self {
            ValueString::Integer(i) => match encode_i64(*i, &mut v) {
                Ok(s) => s,
                Err(e) => match e {
                    EncodeError::IOError(e) => return Err(e),
                    EncodeError::OverflowError => encode_slice_u8(&*self.to_vec(), &mut v, false)?,
                }
            },
            ValueString::Data(d) => encode_slice_u8(&*d, &mut v, true)?,
        };
        let data = [
            vec![TYPE_STRING],
            v,
            vec![(VERSION & 0xff) as u8],
            vec![((VERSION >> 8) & 0xff) as u8],
        ]
        .concat();
        writer.write(&*data)

    }

    pub fn debug_object(&self) -> String {
        let mut serialized_data =  vec![];
        let serialized = self.dump(&mut serialized_data).unwrap();
        let encoding = match self {
            ValueString::Integer(_) => "int",
            ValueString::Data() => "raw",
        };

        format!(
            "Value at: 0x00000000 refcount:1 encoding:{} serialized length: {} lru: 0 \
            lru_seconds_idle: 0",
            encoding, serialized
        )
    }
}