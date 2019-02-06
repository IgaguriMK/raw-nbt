#![doc(html_root_url = "https://docs.rs/raw-nbt/0.1.1")]

pub mod decode;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;

/// An NBT Value.
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
    /// Empty byte list.
    ///
    /// EmptyByteList variant is distinguished from ByteList because it is often used as empty other types.
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

impl Value {
    pub fn byte(&self) -> Result<i8> {
        match self {
            Value::Byte(x) => Ok(*x),
            _ => Err(Error::InvalidType("byte", self.val_type())),
        }
    }

    pub fn short(&self) -> Result<i16> {
        match self {
            Value::Short(x) => Ok(*x),
            _ => Err(Error::InvalidType("short", self.val_type())),
        }
    }

    pub fn int(&self) -> Result<i32> {
        match self {
            Value::Int(x) => Ok(*x),
            _ => Err(Error::InvalidType("int", self.val_type())),
        }
    }

    pub fn long(&self) -> Result<i64> {
        match self {
            Value::Long(x) => Ok(*x),
            _ => Err(Error::InvalidType("long", self.val_type())),
        }
    }

    pub fn float(&self) -> Result<f32> {
        match self {
            Value::Float(x) => Ok(*x),
            _ => Err(Error::InvalidType("float", self.val_type())),
        }
    }

    pub fn double(&self) -> Result<f64> {
        match self {
            Value::Double(x) => Ok(*x),
            _ => Err(Error::InvalidType("double", self.val_type())),
        }
    }

    pub fn byte_array(&self) -> Result<&Vec<i8>> {
        match self {
            Value::ByteArray(x) => Ok(x),
            _ => Err(Error::InvalidType("byte array", self.val_type())),
        }
    }

    pub fn str(&self) -> Result<&str> {
        match self {
            Value::Str(x) => Ok(x),
            _ => Err(Error::InvalidType("str", self.val_type())),
        }
    }

    pub fn compound(&self) -> Result<&Compound> {
        match self {
            Value::Compound(x) => Ok(x),
            _ => Err(Error::InvalidType("compound", self.val_type())),
        }
    }

    /// Get value of compound.
    pub fn get(&self, name: &str) -> Result<&Value> {
        self.compound()?.get(name).ok_or_else(|| Error::NotFound(name.to_string()))
    }

    pub fn int_array(&self) -> Result<&Vec<i32>> {
        match self {
            Value::IntArray(x) => Ok(x),
            _ => Err(Error::InvalidType("int array", self.val_type())),
        }
    }

    pub fn long_array(&self) -> Result<&Vec<i64>> {
        match self {
            Value::LongArray(x) => Ok(x),
            _ => Err(Error::InvalidType("long array", self.val_type())),
        }
    }

    pub fn byte_list(&self) -> Result<Cow<Vec<i8>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::ByteList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("byte list", self.val_type())),
        }
    }

    pub fn short_list(&self) -> Result<Cow<Vec<i16>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::ShortList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("short list", self.val_type())),
        }
    }

    pub fn int_list(&self) -> Result<Cow<Vec<i32>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::IntList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("short list", self.val_type())),
        }
    }

    pub fn long_list(&self) -> Result<Cow<Vec<i64>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::LongList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("long list", self.val_type())),
        }
    }

    pub fn float_list(&self) -> Result<Cow<Vec<f32>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::FloatList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("float list", self.val_type())),
        }
    }

    pub fn double_list(&self) -> Result<Cow<Vec<f64>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::DoubleList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("double list", self.val_type())),
        }
    }

    pub fn byte_array_list(&self) -> Result<Cow<Vec<Vec<i8>>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::ByteArrayList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("byte array list", self.val_type())),
        }
    }

    pub fn str_list(&self) -> Result<Cow<Vec<String>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::StrList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("str list", self.val_type())),
        }
    }

    pub fn list_list(&self) -> Result<Cow<Vec<Value>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::ListList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("list list", self.val_type())),
        }
    }

    pub fn compound_list(&self) -> Result<Cow<Vec<Compound>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::CompoundList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("compound list", self.val_type())),
        }
    }

    pub fn int_array_list(&self) -> Result<Cow<Vec<Vec<i32>>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::IntArrayList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("int array list", self.val_type())),
        }
    }

    pub fn long_array_list(&self) -> Result<Cow<Vec<Vec<i64>>>> {
        match self {
            Value::EndList => Ok(Cow::Owned(Vec::new())),
            Value::EmptyByteList => Ok(Cow::Owned(Vec::new())),
            Value::LongArrayList(x) => Ok(Cow::Borrowed(x)),
            _ => Err(Error::InvalidType("long array list", self.val_type())),
        }
    }

    fn val_type(&self) -> &'static str {
        match self {
            Value::Byte(_) => "byte",
            Value::Short(_) => "short",
            Value::Int(_) => "int",
            Value::Long(_) => "long",
            Value::Float(_) => "float",
            Value::Double(_) => "double",
            Value::ByteArray(_) => "byte array",
            Value::Str(_) => "str",
            Value::Compound(_) => "compound",
            Value::IntArray(_) => "int array",
            Value::LongArray(_) => "long array",
            Value::EndList => "end list",
            Value::EmptyByteList => "byte list (empty)",
            Value::ByteList(_) => "byte list (non-empty)",
            Value::ShortList(_) => "short list",
            Value::IntList(_) => "int list",
            Value::LongList(_) => "long list",
            Value::FloatList(_) => "float list",
            Value::DoubleList(_) => "double list",
            Value::ByteArrayList(_) => "byte array list",
            Value::StrList(_) => "str list",
            Value::ListList(_) => "list list",
            Value::CompoundList(_) => "compound list",
            Value::IntArrayList(_) => "int array list",
            Value::LongArrayList(_) => "long array list",
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub type Compound = BTreeMap<String, Value>;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidType(&'static str, &'static str),
    NotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidType(to_be, actual) => write!(f, "invalid type: wanted '{}' but actual '{}'", to_be, actual),
            Error::NotFound(name) => write!(f, "field not found '{}'", name),
        }
    }
}

impl std::error::Error for Error {}