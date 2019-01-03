use crate::units::{Angle, AngularVelocity, Radians, Degrees, DegreesPerSecond, RadiansPerSecond};
use super::{CodecError};

use protobuf::{CodedOutputStream, RepeatedField};

use krpc::schema::{List, Dictionary, Set, DictionaryEntry, Tuple};

use std::f32::NAN as NAN_f32;
use std::f64::NAN as NAN_f64;
use std::collections::{BTreeMap, HashMap, HashSet, BTreeSet};
use std::hash::{Hash};

use num_traits::{Num, Float};

use uom::Conversion;
use uom::typenum::{Integer};
use uom::si::{Quantity, Dimension, SI};

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

impl Encode for Option<f32> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_float_no_tag(self.unwrap_or(NAN_f32))?))
    }
}

impl Encode for Option<f64> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_double_no_tag(self.unwrap_or(NAN_f64))?))
    }
}

impl Encode for String {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_string_no_tag(self)?))
    }
}

impl Encode for Vec<u8> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_bytes_no_tag(self)?))
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

impl<T : Encode + Eq + Hash> Encode for HashSet<T> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut encoded_set = Vec::with_capacity(self.len());

            for entry in self {
                encoded_set.push(entry.encode()?);
            }

            let mut set = Set::new();
            set.set_items(RepeatedField::from_vec(encoded_set));
            cos.write_message_no_tag(&set)?;
            Ok(())
        })
    }
}

impl<T : Encode + Ord> Encode for BTreeSet<T> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut encoded_set = Vec::with_capacity(self.len());

            for entry in self {
                encoded_set.push(entry.encode()?);
            }

            let mut set = Set::new();
            set.set_items(RepeatedField::from_vec(encoded_set));
            cos.write_message_no_tag(&set)?;
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

impl<T: Float + Encode> Encode for Degrees<T> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        self.scalar().encode()
    }
}

impl<T: Float + Encode> Encode for Option<Degrees<T>>
    where Option<T> : Encode {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        self.as_ref().map(Degrees::scalar).encode()
    }
}

impl<T: Float + Encode> Encode for Radians<T> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        self.scalar().encode()
    }
}

impl<T: Float + Encode> Encode for Option<Radians<T>>
    where Option<T> : Encode {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        self.as_ref().map(Radians::scalar).encode()
    }
}


impl Encode for DegreesPerSecond<f64> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        self.scalar().encode()
    }
}

impl Encode for DegreesPerSecond<f32> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        self.scalar().encode()
    }
}

impl Encode for RadiansPerSecond<f64> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        self.scalar().encode()
    }
}

impl Encode for RadiansPerSecond<f32> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        self.scalar().encode()
    }
}

impl<L : Integer,
    M : Integer,
    T : Integer,
    I : Integer,
    Th : Integer,
    N : Integer,
    J : Integer,
    K : ?Sized,
    V : Conversion<V> + Num + Encode> Encode for Quantity<Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = K>, SI<V>, V> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        self.value.encode()
    }
}

impl<L : Integer,
    M : Integer,
    T : Integer,
    I : Integer,
    Th : Integer,
    N : Integer,
    J : Integer,
    K : ?Sized,
    V : Conversion<V> + Num + Encode + Clone> Encode for Option<Quantity<Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = K>, SI<V>, V>>
    where Option<V> : Encode {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        self.as_ref().map(|q| q.value.clone()).encode()
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
