use crate::{RemoteObject, RemoteEnum, KrpcClient};
use super::{CodecError};

use protobuf::{CodedInputStream};

use krpc::schema::{List, Dictionary, Tuple};

use std::rc::{Rc};
use std::cell::{RefCell};
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash};

use angular_units::{Rad, Deg};

use uom::si::f64::{Length, Velocity, Time, Force, Mass};
use uom::si::mass::{kilogram};
use uom::si::length::{meter};
use uom::si::velocity::{meter_per_second};
use uom::si::time::{second};
use uom::si::force::{newton};


pub trait Decode where Self : Sized {
    fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError>;
}

impl Decode for bool {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_bool()?))
    }
}

impl Decode for i32 {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_sint32()?))
    }
}

impl Decode for i64 {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_sint64()?))
    }
}

impl Decode for u32 {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_uint32()?))
    }
}

impl Decode for u64 {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_uint64()?))
    }
}

impl Decode for f32 {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_float()?))
    }
}

impl Decode for f64 {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_double()?))
    }
}

impl Decode for String {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_string()?))
    }
}

impl<T : Decode> Decode for Vec<T> {
    fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let entries: List = cis.read_message()?;
            let mut decoded_entries = Vec::with_capacity(entries.items.len());

            for entry in entries.items.iter() {
                decoded_entries.push(decode(entry, client)?);
            }
            Ok(decoded_entries)
        })
    }
}

impl<K : Decode + Ord, V : Decode> Decode for BTreeMap<K, V> {
    fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let entries: Dictionary = cis.read_message()?;
            let mut decoded_map = BTreeMap::new();

            for entry in entries.entries.iter() {
                decoded_map.insert(decode(&entry.key, client)?, decode(&entry.value, client)?);
            }
            Ok(decoded_map)
        })
    }
}

impl<K : Decode + Eq + Hash, V : Decode> Decode for HashMap<K, V> {
    fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let entries: Dictionary = cis.read_message()?;
            let mut decoded_map = HashMap::new();

            for entry in entries.entries.iter() {
                decoded_map.insert(decode(&entry.key, client)?, decode(&entry.value, client)?);
            }
            Ok(decoded_map)
        })
    }
}

impl<T1 : Decode, T2 : Decode> Decode for (T1, T2) {
    fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let tuple: Tuple = cis.read_message()?;

            if tuple.items.len() != 2 {
                return Err(CodecError::mismatched_tuple_length(tuple.items.len(), 3))
            }

            Ok((decode(&tuple.get_items()[0], client)?,
                decode(&tuple.get_items()[1], client)?))
        })
    }
}

impl<T1 : Decode, T2 : Decode, T3 : Decode> Decode for (T1, T2, T3) {
    fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let tuple: Tuple = cis.read_message()?;

            if tuple.items.len() != 3 {
                return Err(CodecError::mismatched_tuple_length(tuple.items.len(), 3))
            }

            Ok((decode(&tuple.get_items()[0], client)?,
                decode(&tuple.get_items()[1], client)?,
                decode(&tuple.get_items()[2], client)?))
        })
    }
}

impl<T1 : Decode, T2 : Decode, T3 : Decode, T4 : Decode> Decode for (T1, T2, T3, T4) {
    fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let tuple: Tuple = cis.read_message()?;

            if tuple.items.len() != 3 {
                return Err(CodecError::mismatched_tuple_length(tuple.items.len(), 3))
            }

            Ok((decode(&tuple.get_items()[0], client)?,
                decode(&tuple.get_items()[1], client)?,
                decode(&tuple.get_items()[2], client)?,
                decode(&tuple.get_items()[3], client)?))
        })
    }
}

impl Decode for Deg<f64> {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(Deg(cis.read_double()?)))
    }
}

impl Decode for Deg<f32> {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(Deg(cis.read_float()?)))
    }
}

impl Decode for Rad<f64> {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(Rad(cis.read_double()?)))
    }
}

impl Decode for Rad<f32> {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(Rad(cis.read_float()?)))
    }
}

impl Decode for Length {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(Length::new::<meter>(cis.read_double()?)))
    }
}

impl Decode for Time {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(Time::new::<second>(cis.read_double()?)))
    }
}

impl Decode for Velocity {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(Velocity::new::<meter_per_second>(cis.read_double()?)))
    }
}

impl Decode for Force {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(Force::new::<newton>(cis.read_double()?)))
    }
}

impl Decode for Mass {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(Mass::new::<kilogram>(cis.read_double()?)))
    }
}

pub fn decode_remote_obj<T: RemoteObject>(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<T, CodecError> {
    decode_with(bytes, |cis| {
        match cis.read_uint64()? {
            0 => Err(CodecError::NullValue),
            id => Ok(T::new(Rc::clone(client), id))
        }
    })
}

pub fn decode_remote_obj_opt<T: RemoteObject>(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Option<T>, CodecError> {
    decode_with(bytes, |cis| {
        match cis.read_uint64()? {
            0 => Ok(None),
            id => Ok(Some(T::new(Rc::clone(client), id)))
        }
    })
}

pub fn decode_remote_enum<T: RemoteEnum>(bytes: &Vec<u8>) -> Result<T, CodecError> {
    decode_with(bytes, |cis| {
        let value = cis.read_sint64()?;
        T::from_value(value)
            .ok_or(CodecError::InvalidEnumValue(value))
    })
}

pub fn decode<T: Decode>(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<T, CodecError> {
    T::decode(bytes, client)
}

fn decode_with<T, F>(bytes: &Vec<u8>, decoder : F) -> Result<T, CodecError>
    where F: FnOnce(&mut CodedInputStream) -> Result<T, CodecError> {
    let mut cis = CodedInputStream::from_bytes(bytes);
    decoder(&mut cis)
}