use crate::{RemoteObject, RemoteEnum};
use super::{CodecError};

use protobuf::{CodedOutputStream, RepeatedField};

use krpc::schema::{List, Dictionary, DictionaryEntry, Tuple};

use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash};

use angular_units::{Angle, Rad, Deg};

use uom::si::f64::{Length, Velocity, Time, Force, Mass};
use uom::si::mass::{kilogram};
use uom::si::length::{meter};
use uom::si::velocity::{meter_per_second};
use uom::si::time::{second};
use uom::si::force::{newton};

pub trait Encode {
    fn encode(&self) -> Result<Vec<u8>, CodecError>;
}

impl Encode for bool {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_bool_no_tag(*self)?))
    }
}

impl Encode for i32 {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_sint32_no_tag(*self)?))
    }
}

impl Encode for i64 {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_sint64_no_tag(*self)?))
    }
}

impl Encode for u32 {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_uint32_no_tag(*self)?))
    }
}

impl Encode for u64 {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_uint64_no_tag(*self)?))
    }
}

impl Encode for f32 {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_float_no_tag(*self)?))
    }
}

impl Encode for f64 {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_double_no_tag(*self)?))
    }
}

impl Encode for String {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_string_no_tag(self)?))
    }
}

impl Encode for Deg<f64> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_double_no_tag(self.scalar())?))
    }
}

impl Encode for Deg<f32> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_float_no_tag(self.scalar())?))
    }
}

impl Encode for Rad<f64> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_double_no_tag(self.scalar())?))
    }
}

impl Encode for Rad<f32> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_float_no_tag(self.scalar())?))
    }
}

impl Encode for Length {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_double_no_tag(self.get::<meter>())?))
    }
}

impl Encode for Time {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_double_no_tag(self.get::<second>())?))
    }
}

impl Encode for Velocity {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_double_no_tag(self.get::<meter_per_second>())?))
    }
}

impl Encode for Force {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_double_no_tag(self.get::<newton>())?))
    }
}

impl Encode for Mass {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_double_no_tag(self.get::<kilogram>())?))
    }
}

impl<T : Encode> Encode for Vec<T> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut list = List::new();
            let mut encoded_vec = Vec::with_capacity(self.len());

            for entry in self.iter() {
                encoded_vec.push(entry.encode()?);
            }

            list.set_items(RepeatedField::from_vec(encoded_vec));
            cos.write_message_no_tag(&list)?;
            Ok(())
        })
    }
}

impl<K : Encode + Ord, V : Encode> Encode for BTreeMap<K, V> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut encoded_vec = Vec::with_capacity(self.len());

            for (key, value) in &*self {
                let mut entry = DictionaryEntry::new();
                entry.set_key(key.encode()?);
                entry.set_value(value.encode()?);

                encoded_vec.push(entry);
            }

            let mut dict = Dictionary::new();
            dict.set_entries(RepeatedField::from_vec(encoded_vec));
            cos.write_message_no_tag(&dict)?;
            Ok(())
        })
    }
}

impl<K : Encode + Eq + Hash, V : Encode> Encode for HashMap<K, V> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut encoded_vec = Vec::with_capacity(self.len());

            for (key, value) in &*self {
                let mut entry = DictionaryEntry::new();
                entry.set_key(key.encode()?);
                entry.set_value(value.encode()?);

                encoded_vec.push(entry);
            }

            let mut dict = Dictionary::new();
            dict.set_entries(RepeatedField::from_vec(encoded_vec));
            cos.write_message_no_tag(&dict)?;
            Ok(())
        })
    }
}

impl<T1 : Encode, T2 : Encode> Encode for (T1, T2) {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let (a, b) = self;
            let entries = vec![
                a.encode()?,
                b.encode()?
            ];

            let mut tuple = Tuple::new();
            tuple.set_items(RepeatedField::from_vec(entries));
            cos.write_message_no_tag(&tuple)?;
            Ok(())
        })
    }
}

impl<T1 : Encode, T2 : Encode, T3 : Encode> Encode for (T1, T2, T3) {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let (a, b, c) = self;
            let entries = vec![
                a.encode()?,
                b.encode()?,
                c.encode()?
            ];

            let mut tuple = Tuple::new();
            tuple.set_items(RepeatedField::from_vec(entries));
            cos.write_message_no_tag(&tuple)?;
            Ok(())
        })
    }
}

impl<T1 : Encode, T2 : Encode, T3 : Encode, T4 : Encode> Encode for (T1, T2, T3, T4) {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let (a, b, c, d) = self;
            let entries = vec![
                a.encode()?,
                b.encode()?,
                c.encode()?,
                d.encode()?
            ];

            let mut tuple = Tuple::new();
            tuple.set_items(RepeatedField::from_vec(entries));
            cos.write_message_no_tag(&tuple)?;
            Ok(())
        })
    }
}

fn encode_with<F>(encoder: F) -> Result<Vec<u8>, CodecError>
    where F: FnOnce(&mut CodedOutputStream) -> Result<(), CodecError> {

    let mut encoded = vec!();
    let mut cos = CodedOutputStream::vec(&mut encoded);
    encoder(&mut cos)?;
    cos.flush()?;

    Ok(encoded)
}

pub fn encode_remote_obj<T: RemoteObject>(obj: &T) -> Result<Vec<u8>, CodecError> {
    encode_with(|cos| {
        Ok(cos.write_uint64_no_tag(obj.id())?)
    })
}

pub fn encode_remote_obj_opt<T: RemoteObject>(obj: &Option<T>) -> Result<Vec<u8>, CodecError> {
    encode_with(|cos| {
        Ok(cos.write_uint64_no_tag(obj.as_ref().map( T::id).unwrap_or(0))?)
    })
}

pub fn encode_remote_enum<T: RemoteEnum>(obj: &T) -> Result<Vec<u8>, CodecError> {
    encode_with(|cos| {
        Ok(cos.write_sint64_no_tag(obj.value())?)
    })
}