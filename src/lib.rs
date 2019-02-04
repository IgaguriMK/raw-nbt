use std::collections::BTreeMap;
use std::convert::From;
use std::fmt;
use std::io;
use std::io::Read;
use std::string::FromUtf8Error;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    End,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    Str,
    List,
    Compound,
    IntArray,
    LongArray,
}

impl Type {
    fn try_from(byte: u8) -> Result<Type> {
        match byte {
            0 => Ok(Type::End),
            1 => Ok(Type::Byte),
            2 => Ok(Type::Short),
            3 => Ok(Type::Int),
            4 => Ok(Type::Long),
            5 => Ok(Type::Float),
            6 => Ok(Type::Double),
            7 => Ok(Type::ByteArray),
            8 => Ok(Type::Str),
            9 => Ok(Type::List),
            10 => Ok(Type::Compound),
            11 => Ok(Type::IntArray),
            12 => Ok(Type::LongArray),
            _ => Err(ParseError::UnknownTag),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEnd,
    InvalidUTF8(FromUtf8Error),
    UnknownTag,
    ReadError(io::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedEnd => write!(f, "unexpected input end"),
            ParseError::InvalidUTF8(ref cause) => cause.fmt(f),
            ParseError::UnknownTag => write!(f, "found unknown tag"),
            ParseError::ReadError(ref cause) => cause.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::ReadError(ref cause) => Some(cause),
            ParseError::InvalidUTF8(ref cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> ParseError {
        match e.kind() {
            io::ErrorKind::UnexpectedEof => ParseError::UnexpectedEnd,
            _ => ParseError::ReadError(e),
        }
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(e: FromUtf8Error) -> ParseError {
        ParseError::InvalidUTF8(e)
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct Parser<R> {
    r: R,
}

impl<R: Read> Parser<R> {
    pub fn new(r: R) -> Parser<R> {
        Parser { r }
    }

    pub fn parse(&mut self) -> Result<Value> {
        let mut root = Compound::new();

        while let Some(tag) = self.read_tag()? {
            let name = self.read_str()?;
            let payload = self.parse_value_with_tag(tag)?;
            root.insert(name, payload);
        }

        Ok(Value::Compound(root))
    }

    //// parse ////

    fn parse_value_with_tag(&mut self, tag: Type) -> Result<Value> {
        match tag {
            Type::End => Err(ParseError::UnexpectedEnd),
            Type::Byte => self.parse_byte(),
            Type::Short => self.parse_short(),
            Type::Int => self.parse_int(),
            Type::Long => self.parse_long(),
            Type::Float => self.parse_float(),
            Type::Double => self.parse_double(),
            Type::ByteArray => self.parse_byte_array(),
            Type::Str => self.parse_str(),
            Type::List => self.parse_list(),
            Type::Compound => self.parse_compound(),
            Type::IntArray => self.parse_int_array(),
            Type::LongArray => self.parse_long_array(),
        }
    }

    /// parse subtypes

    fn parse_byte(&mut self) -> Result<Value> {
        Ok(Value::Byte(self.read_byte()?))
    }

    fn parse_short(&mut self) -> Result<Value> {
        Ok(Value::Short(self.read_short()?))
    }

    fn parse_int(&mut self) -> Result<Value> {
        Ok(Value::Int(self.read_int()?))
    }

    fn parse_long(&mut self) -> Result<Value> {
        Ok(Value::Long(self.read_long()?))
    }

    fn parse_float(&mut self) -> Result<Value> {
        Ok(Value::Float(self.read_float()?))
    }

    fn parse_double(&mut self) -> Result<Value> {
        Ok(Value::Double(self.read_double()?))
    }

    fn parse_byte_array(&mut self) -> Result<Value> {
        Ok(Value::ByteArray(self.read_byte_array()?))
    }

    fn parse_str(&mut self) -> Result<Value> {
        Ok(Value::Str(self.read_str()?))
    }

    fn parse_compound(&mut self) -> Result<Value> {
        Ok(Value::Compound(self.read_compound()?))
    }

    fn parse_int_array(&mut self) -> Result<Value> {
        Ok(Value::IntArray(self.read_int_array()?))
    }

    fn parse_long_array(&mut self) -> Result<Value> {
        Ok(Value::LongArray(self.read_long_array()?))
    }

    //// list ////

    fn parse_list(&mut self) -> Result<Value> {
        if let Some(tag) = self.read_tag()? {
            let size = self.read_int()? as usize;

            match tag {
                Type::End => Ok(Value::EndList),
                Type::Byte => {
                    if size == 0 {
                        Ok(Value::EmptyByteList)
                    } else {
                        self.parse_byte_list(size)
                    }
                }
                Type::Short => self.parse_short_list(size),
                Type::Int => self.parse_int_list(size),
                Type::Long => self.parse_long_list(size),
                Type::Float => self.parse_float_list(size),
                Type::Double => self.parse_double_list(size),
                Type::ByteArray => self.parse_byte_array_list(size),
                Type::Str => self.parse_str_list(size),
                Type::List => self.parse_list_list(size),
                Type::Compound => self.parse_compound_list(size),
                Type::IntArray => self.parse_int_array_list(size),
                Type::LongArray => self.parse_long_array_list(size),
            }
        } else {
            Err(ParseError::UnexpectedEnd)
        }
    }

    fn parse_byte_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<i8> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_byte()?);
        }
        Ok(Value::ByteList(list))
    }

    fn parse_short_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<i16> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_short()?);
        }
        Ok(Value::ShortList(list))
    }

    fn parse_int_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<i32> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_int()?);
        }
        Ok(Value::IntList(list))
    }

    fn parse_long_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_long()?);
        }
        Ok(Value::LongList(list))
    }

    fn parse_float_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<f32> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_float()?);
        }
        Ok(Value::FloatList(list))
    }

    fn parse_double_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<f64> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_double()?);
        }
        Ok(Value::DoubleList(list))
    }

    fn parse_byte_array_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<Vec<i8>> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_byte_array()?);
        }
        Ok(Value::ByteArrayList(list))
    }

    fn parse_str_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<String> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_str()?);
        }
        Ok(Value::StrList(list))
    }

    fn parse_list_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<Value> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.parse_list()?);
        }
        Ok(Value::ListList(list))
    }

    fn parse_compound_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<Compound> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_compound()?);
        }
        Ok(Value::CompoundList(list))
    }

    fn parse_int_array_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<Vec<i32>> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_int_array()?);
        }
        Ok(Value::IntArrayList(list))
    }

    fn parse_long_array_list(&mut self, size: usize) -> Result<Value> {
        let mut list: Vec<Vec<i64>> = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_long_array()?);
        }
        Ok(Value::LongArrayList(list))
    }

    //// read ////

    fn read_tag(&mut self) -> Result<Option<Type>> {
        let mut bs: [u8; 1] = [0; 1];

        match self.r.read_exact(&mut bs) {
            Ok(()) => Ok(Some(Type::try_from(bs[0])?)),
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    Ok(None)
                } else {
                    Err(ParseError::from(e))
                }
            }
        }
    }

    fn read_byte(&mut self) -> Result<i8> {
        let mut bs = [0u8; 1];
        self.r.read_exact(&mut bs)?;
        Ok(i8::from_be_bytes(bs))
    }

    fn read_short(&mut self) -> Result<i16> {
        let mut bs = [0u8; 2];
        self.r.read_exact(&mut bs)?;
        Ok(i16::from_be_bytes(bs))
    }

    fn read_int(&mut self) -> Result<i32> {
        let mut bs = [0u8; 4];
        self.r.read_exact(&mut bs)?;
        Ok(i32::from_be_bytes(bs))
    }

    fn read_long(&mut self) -> Result<i64> {
        let mut bs = [0u8; 8];
        self.r.read_exact(&mut bs)?;
        Ok(i64::from_be_bytes(bs))
    }

    fn read_float(&mut self) -> Result<f32> {
        let mut bs = [0u8; 4];
        self.r.read_exact(&mut bs)?;
        let x = u32::from_be_bytes(bs);
        Ok(f32::from_bits(x))
    }

    fn read_double(&mut self) -> Result<f64> {
        let mut bs = [0u8; 8];
        self.r.read_exact(&mut bs)?;
        let x = u64::from_be_bytes(bs);
        Ok(f64::from_bits(x))
    }

    fn read_byte_array(&mut self) -> Result<Vec<i8>> {
        let size = self.read_int()? as usize;
        let mut arr: Vec<i8> = Vec::with_capacity(size);

        for _ in 0..size {
            arr.push(self.read_byte()?);
        }

        Ok(arr)
    }

    fn read_str(&mut self) -> Result<String> {
        let size = self.read_short()? as usize;

        let mut bs = vec![0u8; size];
        self.r.read_exact(bs.as_mut_slice())?;

        Ok(String::from_utf8(bs)?)
    }

    fn read_compound(&mut self) -> Result<Compound> {
        let mut root = Compound::new();

        loop {
            if let Some(tag) = self.read_tag()? {
                if tag == Type::End {
                    return Ok(root);
                }

                let name = self.read_str()?;
                let payload = self.parse_value_with_tag(tag)?;
                root.insert(name, payload);
            } else {
                return Err(ParseError::UnexpectedEnd);
            }
        }
    }

    fn read_int_array(&mut self) -> Result<Vec<i32>> {
        let size = self.read_int()? as usize;
        let mut arr: Vec<i32> = Vec::with_capacity(size);

        for _ in 0..size {
            arr.push(self.read_int()?);
        }

        Ok(arr)
    }

    fn read_long_array(&mut self) -> Result<Vec<i64>> {
        let size = self.read_int()? as usize;
        let mut arr: Vec<i64> = Vec::with_capacity(size);

        for _ in 0..size {
            arr.push(self.read_long()?);
        }

        Ok(arr)
    }
}
