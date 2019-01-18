use super::CodecError;
use crate::client::schema::{Dictionary, List, Set, Tuple};
use crate::Connection;

use protobuf::CodedInputStream;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

pub trait Decode
where
    Self: Sized,
{
    fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError>;
}

impl Decode for bool {
    fn decode(bytes: &Vec<u8>, _connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_bool()?))
    }
}

impl Decode for i32 {
    fn decode(bytes: &Vec<u8>, _connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_sint32()?))
    }
}

impl Decode for i64 {
    fn decode(bytes: &Vec<u8>, _connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_sint64()?))
    }
}

impl Decode for u32 {
    fn decode(bytes: &Vec<u8>, _connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_uint32()?))
    }
}

impl Decode for u64 {
    fn decode(bytes: &Vec<u8>, _connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_uint64()?))
    }
}

impl Decode for f32 {
    fn decode(bytes: &Vec<u8>, _connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_float()?))
    }
}

impl Decode for f64 {
    fn decode(bytes: &Vec<u8>, _connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_double()?))
    }
}

impl Decode for String {
    fn decode(bytes: &Vec<u8>, _connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_string()?))
    }
}

impl Decode for Vec<u8> {
    fn decode(bytes: &Vec<u8>, _connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_bytes()?))
    }
}

impl<'a, T: Decode> Decode for Vec<T> {
    fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let entries: List = cis.read_message()?;
            let mut decoded_entries = Vec::with_capacity(entries.items.len());

            for entry in entries.items.iter() {
                decoded_entries.push(decode(entry, connection.clone())?);
            }
            Ok(decoded_entries)
        })
    }
}

impl<'a, K: Decode + Ord, V: Decode> Decode for BTreeMap<K, V> {
    fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let entries: Dictionary = cis.read_message()?;
            let mut decoded_map = BTreeMap::new();

            for entry in entries.entries.iter() {
                decoded_map.insert(
                    decode(&entry.key, connection.clone())?,
                    decode(&entry.value, connection.clone())?,
                );
            }
            Ok(decoded_map)
        })
    }
}

impl<'a, K: Decode + Eq + Hash, V: Decode> Decode for HashMap<K, V> {
    fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let entries: Dictionary = cis.read_message()?;
            let mut decoded_map = HashMap::new();

            for entry in entries.entries.iter() {
                decoded_map.insert(
                    decode(&entry.key, connection.clone())?,
                    decode(&entry.value, connection.clone())?,
                );
            }
            Ok(decoded_map)
        })
    }
}

impl<'a, T: Decode + Eq + Hash> Decode for HashSet<T> {
    fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let entries: Set = cis.read_message()?;
            let mut decoded_set = HashSet::with_capacity(entries.items.len());

            for entry in entries.items.iter() {
                decoded_set.insert(decode(entry, connection.clone())?);
            }
            Ok(decoded_set)
        })
    }
}

impl<'a, T: Decode + Ord> Decode for BTreeSet<T> {
    fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let entries: Set = cis.read_message()?;
            let mut decoded_set = BTreeSet::new();

            for entry in entries.items.iter() {
                decoded_set.insert(decode(entry, connection.clone())?);
            }
            Ok(decoded_set)
        })
    }
}

impl<'a, T1: Decode, T2: Decode> Decode for (T1, T2) {
    fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let tuple: Tuple = cis.read_message()?;

            if tuple.items.len() != 2 {
                return Err(CodecError::mismatched_tuple_length(tuple.items.len(), 3));
            }

            Ok((
                decode(&tuple.get_items()[0], connection.clone())?,
                decode(&tuple.get_items()[1], connection.clone())?,
            ))
        })
    }
}

impl<'a, T1: Decode, T2: Decode, T3: Decode> Decode for (T1, T2, T3) {
    fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let tuple: Tuple = cis.read_message()?;

            if tuple.items.len() != 3 {
                return Err(CodecError::mismatched_tuple_length(tuple.items.len(), 3));
            }

            Ok((
                decode(&tuple.get_items()[0], connection.clone())?,
                decode(&tuple.get_items()[1], connection.clone())?,
                decode(&tuple.get_items()[2], connection.clone())?,
            ))
        })
    }
}

impl<'a, T1: Decode, T2: Decode, T3: Decode, T4: Decode> Decode for (T1, T2, T3, T4) {
    fn decode(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let tuple: Tuple = cis.read_message()?;

            if tuple.items.len() != 3 {
                return Err(CodecError::mismatched_tuple_length(tuple.items.len(), 3));
            }

            Ok((
                decode(&tuple.get_items()[0], connection.clone())?,
                decode(&tuple.get_items()[1], connection.clone())?,
                decode(&tuple.get_items()[2], connection.clone())?,
                decode(&tuple.get_items()[3], connection.clone())?,
            ))
        })
    }
}

pub fn decode<T: Decode>(bytes: &Vec<u8>, connection: Rc<Connection>) -> Result<T, CodecError> {
    T::decode(bytes, connection)
}

pub fn decode_message<T: protobuf::Message>(bytes: &Vec<u8>) -> Result<T, CodecError> {
    decode_with(bytes, |cis| Ok(cis.read_message()?))
}

fn decode_with<T, F>(bytes: &Vec<u8>, decoder: F) -> Result<T, CodecError>
where
    F: FnOnce(&mut CodedInputStream) -> Result<T, CodecError>,
{
    let mut cis = CodedInputStream::from_bytes(bytes);
    decoder(&mut cis)
}
