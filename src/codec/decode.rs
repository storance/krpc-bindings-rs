use super::{CodecError, CodecResult};
use crate::client::schema::{Dictionary, Event, List, Services, Set, Status, Stream, Tuple};
use crate::client::Connection;

use protobuf::{CodedInputStream, Message};

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

pub trait Decode<'a>
where
    Self: 'a + Sized,
{
    fn decode(bytes: &Vec<u8>, connection: &'a Connection) -> CodecResult<Self>;
}

impl<'a> Decode<'a> for bool {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        decode_with(bytes, |cis| Ok(cis.read_bool()?))
    }
}

impl<'a> Decode<'a> for i32 {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        decode_with(bytes, |cis| Ok(cis.read_sint32()?))
    }
}

impl<'a> Decode<'a> for i64 {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        decode_with(bytes, |cis| Ok(cis.read_sint64()?))
    }
}

impl<'a> Decode<'a> for u32 {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        decode_with(bytes, |cis| Ok(cis.read_uint32()?))
    }
}

impl<'a> Decode<'a> for u64 {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        decode_with(bytes, |cis| Ok(cis.read_uint64()?))
    }
}

impl<'a> Decode<'a> for f32 {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        decode_with(bytes, |cis| Ok(cis.read_float()?))
    }
}

impl<'a> Decode<'a> for f64 {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        decode_with(bytes, |cis| Ok(cis.read_double()?))
    }
}

impl<'a> Decode<'a> for String {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        decode_with(bytes, |cis| Ok(cis.read_string()?))
    }
}

impl<'a> Decode<'a> for Vec<u8> {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        decode_with(bytes, |cis| Ok(cis.read_bytes()?))
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Vec<T> {
    fn decode(bytes: &Vec<u8>, connection: &'a Connection) -> CodecResult<Self> {
        let mut list = List::new();
        list.merge_from_bytes(bytes.as_slice())?;

        let mut decoded_vec = Vec::with_capacity(list.items.len());

        for entry in list.items.iter() {
            decoded_vec.push(T::decode(entry, connection)?);
        }

        Ok(decoded_vec)
    }
}

impl<'a, K: Decode<'a> + Ord, V: Decode<'a>> Decode<'a> for BTreeMap<K, V> {
    fn decode(bytes: &Vec<u8>, connection: &'a Connection) -> CodecResult<Self> {
        let mut dict = Dictionary::new();
        dict.merge_from_bytes(bytes.as_slice())?;

        let mut decoded_map = BTreeMap::new();
        for entry in dict.entries.iter() {
            decoded_map.insert(
                K::decode(&entry.key, connection)?,
                V::decode(&entry.value, connection)?,
            );
        }
        Ok(decoded_map)
    }
}

impl<'a, K: Decode<'a> + Eq + Hash, V: Decode<'a>> Decode<'a> for HashMap<K, V> {
    fn decode(bytes: &Vec<u8>, connection: &'a Connection) -> CodecResult<Self> {
        let mut dict = Dictionary::new();
        dict.merge_from_bytes(bytes.as_slice())?;

        let mut decoded_map = HashMap::with_capacity(dict.entries.len());
        for entry in dict.entries.iter() {
            decoded_map.insert(
                K::decode(&entry.key, connection)?,
                V::decode(&entry.value, connection)?,
            );
        }
        Ok(decoded_map)
    }
}

impl<'a, T: Decode<'a> + Eq + Hash> Decode<'a> for HashSet<T> {
    fn decode(bytes: &Vec<u8>, connection: &'a Connection) -> CodecResult<Self> {
        let mut set = Set::new();
        set.merge_from_bytes(bytes.as_slice())?;

        let mut decoded_set = HashSet::with_capacity(set.items.len());
        for entry in set.items.iter() {
            decoded_set.insert(T::decode(entry, connection)?);
        }
        Ok(decoded_set)
    }
}

impl<'a, T: Decode<'a> + Ord> Decode<'a> for BTreeSet<T> {
    fn decode(bytes: &Vec<u8>, connection: &'a Connection) -> CodecResult<Self> {
        let mut set = Set::new();
        set.merge_from_bytes(bytes.as_slice())?;

        let mut decoded_set = BTreeSet::new();
        for entry in set.items.iter() {
            decoded_set.insert(T::decode(entry, connection)?);
        }
        Ok(decoded_set)
    }
}

impl<'a> Decode<'a> for () {
    fn decode(_bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        Ok(())
    }
}

impl<'a, T1: Decode<'a>> Decode<'a> for (T1,) {
    fn decode(bytes: &Vec<u8>, connection: &'a Connection) -> CodecResult<Self> {
        let mut tuple = Tuple::new();
        tuple.merge_from_bytes(bytes.as_slice())?;

        if tuple.items.len() != 1 {
            Err(CodecError::MismatchedTupleLength {
                actual: tuple.items.len(),
                expected: 1,
            })?;
        }

        Ok((T1::decode(&tuple.get_items()[0], connection)?,))
    }
}

impl<'a, T1: Decode<'a>, T2: Decode<'a>> Decode<'a> for (T1, T2) {
    fn decode(bytes: &Vec<u8>, connection: &'a Connection) -> CodecResult<Self> {
        let mut tuple = Tuple::new();
        tuple.merge_from_bytes(bytes.as_slice())?;

        if tuple.items.len() != 2 {
            Err(CodecError::MismatchedTupleLength {
                actual: tuple.items.len(),
                expected: 2,
            })?;
        }

        Ok((
            T1::decode(&tuple.get_items()[0], connection)?,
            T2::decode(&tuple.get_items()[1], connection)?,
        ))
    }
}

impl<'a, T1: Decode<'a>, T2: Decode<'a>, T3: Decode<'a>> Decode<'a> for (T1, T2, T3) {
    fn decode(bytes: &Vec<u8>, connection: &'a Connection) -> CodecResult<Self> {
        let mut tuple = Tuple::new();
        tuple.merge_from_bytes(bytes.as_slice())?;

        if tuple.items.len() != 3 {
            Err(CodecError::MismatchedTupleLength {
                actual: tuple.items.len(),
                expected: 3,
            })?;
        }

        Ok((
            T1::decode(&tuple.get_items()[0], connection)?,
            T2::decode(&tuple.get_items()[1], connection)?,
            T3::decode(&tuple.get_items()[2], connection)?,
        ))
    }
}

impl<'a, T1: Decode<'a>, T2: Decode<'a>, T3: Decode<'a>, T4: Decode<'a>> Decode<'a>
    for (T1, T2, T3, T4)
{
    fn decode(bytes: &Vec<u8>, connection: &'a Connection) -> CodecResult<Self> {
        let mut tuple = Tuple::new();
        tuple.merge_from_bytes(bytes.as_slice())?;

        if tuple.items.len() != 4 {
            Err(CodecError::MismatchedTupleLength {
                actual: tuple.items.len(),
                expected: 4,
            })?;
        }

        Ok((
            T1::decode(&tuple.get_items()[0], connection)?,
            T2::decode(&tuple.get_items()[1], connection)?,
            T3::decode(&tuple.get_items()[2], connection)?,
            T4::decode(&tuple.get_items()[3], connection)?,
        ))
    }
}

impl<'a> Decode<'a> for Services {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        let mut value = Self::new();
        value.merge_from_bytes(bytes.as_slice())?;
        Ok(value)
    }
}

impl<'a> Decode<'a> for Status {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        let mut value = Self::new();
        value.merge_from_bytes(bytes.as_slice())?;
        Ok(value)
    }
}

impl<'a> Decode<'a> for Stream {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        let mut value = Self::new();
        value.merge_from_bytes(bytes.as_slice())?;
        Ok(value)
    }
}
impl<'a> Decode<'a> for Event {
    fn decode(bytes: &Vec<u8>, _connection: &'a Connection) -> CodecResult<Self> {
        let mut value = Self::new();
        value.merge_from_bytes(bytes.as_slice())?;
        Ok(value)
    }
}

fn decode_with<T, F>(bytes: &Vec<u8>, decoder: F) -> CodecResult<T>
where
    F: FnOnce(&mut CodedInputStream) -> CodecResult<T>,
{
    let mut cis = CodedInputStream::from_bytes(bytes);
    decoder(&mut cis)
}
