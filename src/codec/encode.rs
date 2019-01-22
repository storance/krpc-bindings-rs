use super::CodecError;
use crate::client::schema::{Dictionary, DictionaryEntry, List, ProcedureCall, Set, Tuple};

use protobuf::{CodedOutputStream, Message};

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

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

impl Encode for &str {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_string_no_tag(self)?))
    }
}

impl Encode for Vec<u8> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(cos.write_bytes_no_tag(self)?))
    }
}

impl Encode for ProcedureCall {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| Ok(self.write_to(cos)?))
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut list = List::new();
            for entry in self.iter() {
                list.items.push(entry.encode()?);
            }
            
            list.write_to(cos)?;
            Ok(())
        })
    }
}

impl<K: Encode + Ord, V: Encode> Encode for BTreeMap<K, V> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut dict = Dictionary::new();
            for (key, value) in &*self {
                let mut entry = DictionaryEntry::new();
                entry.set_key(key.encode()?);
                entry.set_value(value.encode()?);

                dict.entries.push(entry);
            }

            dict.write_to(cos)?;
            Ok(())
        })
    }
}

impl<K: Encode + Eq + Hash, V: Encode> Encode for HashMap<K, V> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut dict = Dictionary::new();
            for (key, value) in &*self {
                let mut entry = DictionaryEntry::new();
                entry.set_key(key.encode()?);
                entry.set_value(value.encode()?);

                dict.entries.push(entry);
            }

            dict.write_to(cos)?;
            Ok(())
        })
    }
}

impl<T: Encode + Eq + Hash> Encode for HashSet<T> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut set = Set::new();

            for entry in self {
                set.items.push(entry.encode()?);
            }

            set.write_to(cos)?;
            Ok(())
        })
    }
}

impl<T: Encode + Ord> Encode for BTreeSet<T> {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut set = Set::new();

            for entry in self {
                set.items.push(entry.encode()?);
            }

            set.write_to(cos)?;
            Ok(())
        })
    }
}

impl<T1: Encode> Encode for (T1,) {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut tuple = Tuple::new();
            tuple.items.push(self.0.encode()?);
            tuple.write_to(cos)?;
            Ok(())
        })
    }
}

impl<T1: Encode, T2: Encode> Encode for (T1, T2) {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut tuple = Tuple::new();
            tuple.items.push(self.0.encode()?);
            tuple.items.push(self.1.encode()?);
            tuple.write_to(cos)?;
            Ok(())
        })
    }
}

impl<T1: Encode, T2: Encode, T3: Encode> Encode for (T1, T2, T3) {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut tuple = Tuple::new();
            tuple.items.push(self.0.encode()?);
            tuple.items.push(self.1.encode()?);
            tuple.items.push(self.2.encode()?);
            tuple.write_to(cos)?;
            Ok(())
        })
    }
}

impl<T1: Encode, T2: Encode, T3: Encode, T4: Encode> Encode for (T1, T2, T3, T4) {
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        encode_with(|cos| {
            let mut tuple = Tuple::new();
            tuple.items.push(self.0.encode()?);
            tuple.items.push(self.1.encode()?);
            tuple.items.push(self.2.encode()?);
            tuple.items.push(self.3.encode()?);
            tuple.write_to(cos)?;
            Ok(())
        })
    }
}

fn encode_with<F>(encoder: F) -> Result<Vec<u8>, CodecError>
where
    F: FnOnce(&mut CodedOutputStream) -> Result<(), CodecError>,
{
    let mut encoded = vec![];
    let mut cos = CodedOutputStream::vec(&mut encoded);
    encoder(&mut cos)?;
    cos.flush()?;

    Ok(encoded)
}
