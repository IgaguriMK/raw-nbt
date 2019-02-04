pub mod decode;

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // basic types
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    Str(String),
    Compound(Compound),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
    // list
    EndList,
    EmptyByteList,
    ByteList(Vec<i8>),
    ShortList(Vec<i16>),
    IntList(Vec<i32>),
    LongList(Vec<i64>),
    FloatList(Vec<f32>),
    DoubleList(Vec<f64>),
    ByteArrayList(Vec<Vec<i8>>),
    StrList(Vec<String>),
    ListList(Vec<Value>),
    CompoundList(Vec<Compound>),
    IntArrayList(Vec<Vec<i32>>),
    LongArrayList(Vec<Vec<i64>>),
}

pub type Compound = BTreeMap<String, Value>;

