use crate::{KrpcClient};
use crate::units::{Angle, AngularVelocity, Degrees, DegreesPerSecond, Radians, RadiansPerSecond};
use super::{CodecError};

use protobuf::{CodedInputStream};

use krpc::schema::{List, Dictionary, Tuple, Set};

use std::rc::{Rc};
use std::cell::{RefCell};
use std::collections::{BTreeMap, HashMap, BTreeSet, HashSet};
use std::hash::{Hash};

use core::marker::{PhantomData};

use uom::si;


pub trait Decode where Self : Sized {
    fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError>;
}

#[doc(hidden)]
macro_rules! wrapper_decode_impl {
    (|$value: ident : $value_type: ty| $construct: expr) => {
        fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
            let $value: $value_type = decode(bytes, client)?;
            Ok($construct)
        }
    };
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

impl Decode for Vec<u8> {
    fn decode(bytes: &Vec<u8>, _client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| Ok(cis.read_bytes()?))
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

impl<T : Decode + Eq + Hash> Decode for HashSet<T> {
    fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let entries: Set = cis.read_message()?;
            let mut decoded_set = HashSet::with_capacity(entries.items.len());

            for entry in entries.items.iter() {
                decoded_set.insert(decode(entry, client)?);
            }
            Ok(decoded_set)
        })
    }
}

impl<T : Decode + Ord> Decode for BTreeSet<T> {
    fn decode(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<Self, CodecError> {
        decode_with(bytes, |cis| {
            let entries: Set = cis.read_message()?;
            let mut decoded_set = BTreeSet::new();

            for entry in entries.items.iter() {
                decoded_set.insert(decode(entry, client)?);
            }
            Ok(decoded_set)
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

impl Decode for Degrees<f32> {
    wrapper_decode_impl!(|value: f32| Self::new(value));
}

impl Decode for Degrees<f64> {
    wrapper_decode_impl!(|value: f64| Self::new(value));
}

impl Decode for Radians<f32> {
    wrapper_decode_impl!(|value: f32| Self::new(value));
}

impl Decode for Radians<f64> {
    wrapper_decode_impl!(|value: f64| Self::new(value));
}

impl Decode for DegreesPerSecond<f32> {
    wrapper_decode_impl!(|value: f32| Self::new(value));
}

impl Decode for DegreesPerSecond<f64> {
    wrapper_decode_impl!(|value: f64| Self::new(value));
}

impl Decode for RadiansPerSecond<f32> {
    wrapper_decode_impl!(|value: f32| Self::new(value));
}

impl Decode for RadiansPerSecond<f64> {
    wrapper_decode_impl!(|value: f64| Self::new(value));
}

impl<L : uom::typenum::Integer,
    M : uom::typenum::Integer,
    T : uom::typenum::Integer,
    I : uom::typenum::Integer,
    Th : uom::typenum::Integer,
    N : uom::typenum::Integer,
    J : uom::typenum::Integer,
    K : ?Sized> Decode for si::Quantity<si::Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = K>, si::SI<f32>, f32> {
    wrapper_decode_impl!(|value: f32| si::Quantity{
        dimension: PhantomData,
        units: PhantomData,
        value: value
    });
}

impl<L : uom::typenum::Integer,
     M : uom::typenum::Integer,
     T : uom::typenum::Integer,
     I : uom::typenum::Integer,
     Th : uom::typenum::Integer,
     N : uom::typenum::Integer,
     J : uom::typenum::Integer,
     K : ?Sized> Decode for si::Quantity<si::Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = K>, si::SI<f64>, f64> {
    wrapper_decode_impl!(|value: f64| si::Quantity{
        dimension: PhantomData,
        units: PhantomData,
        value: value
    });
}

pub fn decode<T: Decode>(bytes: &Vec<u8>, client: &Rc<RefCell<KrpcClient>>) -> Result<T, CodecError> {
    T::decode(bytes, client)
}

fn decode_with<T, F>(bytes: &Vec<u8>, decoder : F) -> Result<T, CodecError>
    where F: FnOnce(&mut CodedInputStream) -> Result<T, CodecError> {
    let mut cis = CodedInputStream::from_bytes(bytes);
    decoder(&mut cis)
}