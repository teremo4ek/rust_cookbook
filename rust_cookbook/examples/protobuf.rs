use thiserror::Error;

/*
message PhoneNumber {
  optional string number = 1;
  optional string type = 2;
}

message Person {
  optional string name = 1;
  optional int32 id = 2;
  repeated PhoneNumber phones = 3;
} */

const VARINT_MSB: u8 = 0x80;
const VARINT_PAYLOAD: u8 = 0x7F;
const TAG_TYPE_BITS: u32 = 3;
const MAX_VARINT_BYTES: usize = 10;

#[derive(Debug, Error)]
enum ParseError {
    #[error("invalid varint")]
    InvalidVarint,
    #[error("invalid wire type: {0}")]
    InvalidWireType(u64),
    #[error("unexpected end of input: need {need} bytes, have {have}")]
    UnexpectedEof { need: usize, have: usize },
    #[error("invalid UTF-8 string")]
    InvalidString,
    #[error("wrong wire type for {0}")]
    WrongWireType(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WireType {
    Varint,
    Len,
    I32,
}

impl TryFrom<u64> for WireType {
    type Error = ParseError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Varint),
            2 => Ok(Self::Len),
            5 => Ok(Self::I32),
            v => Err(ParseError::InvalidWireType(v)),
        }
    }
}

#[derive(Debug)]
enum FieldValue<'a> {
    Varint(u64),
    Len(&'a [u8]),
    I32(i32),
}

#[derive(Debug)]
struct Field<'a> {
    field_num: u64,
    value: FieldValue<'a>,
}

trait ProtoMessage<'a>: Default {
    fn add_field(&mut self, field: Field<'a>) -> Result<(), ParseError>;
}

impl<'a> FieldValue<'a> {
    fn as_string(&self) -> Result<&'a str, ParseError> {
        let FieldValue::Len(data) = self else {
            return Err(ParseError::WrongWireType("string"));
        };
        std::str::from_utf8(data).map_err(|_| ParseError::InvalidString)
    }

    fn as_bytes(&self) -> Result<&'a [u8], ParseError> {
        let FieldValue::Len(data) = self else {
            return Err(ParseError::WrongWireType("bytes"));
        };
        Ok(data)
    }

    fn as_u64(&self) -> Result<u64, ParseError> {
        let FieldValue::Varint(v) = self else {
            return Err(ParseError::WrongWireType("varint"));
        };
        Ok(*v)
    }
}

// Forward-accumulating varint parser. Standard protobuf supports up to 10 bytes for u64.
fn parse_varint(data: &[u8]) -> Result<(u64, &[u8]), ParseError> {
    let mut value = 0u64;
    let mut shift = 0u32;

    for (i, &byte) in data.iter().enumerate() {
        if i >= MAX_VARINT_BYTES {
            return Err(ParseError::InvalidVarint);
        }
        value |= ((byte & VARINT_PAYLOAD) as u64) << shift;
        if byte & VARINT_MSB == 0 {
            return Ok((value, &data[i + 1..]));
        }
        shift += 7;
    }

    Err(ParseError::InvalidVarint)
}

fn unpack_tag(tag: u64) -> Result<(u64, WireType), ParseError> {
    let field_num = tag >> TAG_TYPE_BITS;
    let wire_type = WireType::try_from(tag & ((1 << TAG_TYPE_BITS) - 1))?;
    Ok((field_num, wire_type))
}

fn parse_field(data: &[u8]) -> Result<(Field<'_>, &[u8]), ParseError> {
    let (tag, rest) = parse_varint(data)?;
    let (field_num, wire_type) = unpack_tag(tag)?;

    let (value, rest) = match wire_type {
        WireType::Varint => {
            let (v, r) = parse_varint(rest)?;
            (FieldValue::Varint(v), r)
        }
        WireType::Len => {
            let (len, r) = parse_varint(rest)?;
            let len = len as usize;
            if r.len() < len {
                return Err(ParseError::UnexpectedEof {
                    need: len,
                    have: r.len(),
                });
            }
            let (bytes, r) = r.split_at(len);
            (FieldValue::Len(bytes), r)
        }
        WireType::I32 => {
            if rest.len() < 4 {
                return Err(ParseError::UnexpectedEof {
                    need: 4,
                    have: rest.len(),
                });
            }
            let (bytes, r) = rest.split_at(4);
            (
                FieldValue::I32(i32::from_le_bytes(bytes.try_into().unwrap())),
                r,
            )
        }
    };

    Ok((Field { field_num, value }, rest))
}

fn parse_message<'a, T: ProtoMessage<'a>>(data: &'a [u8]) -> Result<T, ParseError> {
    let mut result = T::default();
    let mut rest = data;

    while !rest.is_empty() {
        let (field, remaining) = parse_field(rest)?;
        result.add_field(field)?;
        rest = remaining;
    }

    Ok(result)
}

#[derive(Debug, Default)]
struct PhoneNumber<'a> {
    number: &'a str,
    type_: &'a str,
}

#[derive(Debug, Default)]
struct Person<'a> {
    name: &'a str,
    id: u64,
    phone: Vec<PhoneNumber<'a>>,
}

impl<'a> ProtoMessage<'a> for Person<'a> {
    fn add_field(&mut self, field: Field<'a>) -> Result<(), ParseError> {
        match field.field_num {
            1 => self.name = field.value.as_string()?,
            2 => self.id = field.value.as_u64()?,
            3 => self.phone.push(parse_message(field.value.as_bytes()?)?),
            _ => {}
        }
        Ok(())
    }
}

impl<'a> ProtoMessage<'a> for PhoneNumber<'a> {
    fn add_field(&mut self, field: Field<'a>) -> Result<(), ParseError> {
        match field.field_num {
            1 => self.number = field.value.as_string()?,
            2 => self.type_ = field.value.as_string()?,
            _ => {}
        }
        Ok(())
    }
}

fn main() {
    let person: Person = parse_message(&[
        0x0a, 0x07, 0x6d, 0x61, 0x78, 0x77, 0x65, 0x6c, 0x6c, 0x10, 0x2a, 0x1a, 0x16, 0x0a, 0x0e,
        0x2b, 0x31, 0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35, 0x2d, 0x31, 0x32, 0x31, 0x32, 0x12,
        0x04, 0x68, 0x6f, 0x6d, 0x65, 0x1a, 0x18, 0x0a, 0x0e, 0x2b, 0x31, 0x38, 0x30, 0x30, 0x2d,
        0x38, 0x36, 0x37, 0x2d, 0x35, 0x33, 0x30, 0x38, 0x12, 0x06, 0x6d, 0x6f, 0x62, 0x69, 0x6c,
        0x65,
    ])
    .unwrap();
    println!("{:#?}", person);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn as_string() {
        assert!(FieldValue::Varint(10).as_string().is_err());
        assert!(FieldValue::I32(10).as_string().is_err());
        assert_eq!(FieldValue::Len(b"hello").as_string().unwrap(), "hello");
    }

    #[test]
    fn as_bytes() {
        assert!(FieldValue::Varint(10).as_bytes().is_err());
        assert!(FieldValue::I32(10).as_bytes().is_err());
        assert_eq!(FieldValue::Len(b"hello").as_bytes().unwrap(), b"hello");
    }

    #[test]
    fn as_u64() {
        assert_eq!(FieldValue::Varint(10).as_u64().unwrap(), 10u64);
        assert!(FieldValue::I32(10).as_u64().is_err());
        assert!(FieldValue::Len(b"hello").as_u64().is_err());
    }

    #[test]
    fn varint_single_byte() {
        let (val, rest) = parse_varint(&[0x2a]).unwrap();
        assert_eq!(val, 42);
        assert!(rest.is_empty());
    }

    #[test]
    fn varint_multi_byte() {
        let (val, rest) = parse_varint(&[0x96, 0x01]).unwrap();
        assert_eq!(val, 150);
        assert!(rest.is_empty());
    }

    #[test]
    fn parse_person_message() {
        let person: Person = parse_message(&[
            0x0a, 0x07, 0x6d, 0x61, 0x78, 0x77, 0x65, 0x6c, 0x6c, 0x10, 0x2a, 0x1a, 0x16, 0x0a,
            0x0e, 0x2b, 0x31, 0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35, 0x2d, 0x31, 0x32, 0x31,
            0x32, 0x12, 0x04, 0x68, 0x6f, 0x6d, 0x65, 0x1a, 0x18, 0x0a, 0x0e, 0x2b, 0x31, 0x38,
            0x30, 0x30, 0x2d, 0x38, 0x36, 0x37, 0x2d, 0x35, 0x33, 0x30, 0x38, 0x12, 0x06, 0x6d,
            0x6f, 0x62, 0x69, 0x6c, 0x65,
        ])
        .unwrap();

        assert_eq!(person.name, "maxwell");
        assert_eq!(person.id, 42);
        assert_eq!(person.phone.len(), 2);
        assert_eq!(person.phone[0].number, "+1202-555-1212");
        assert_eq!(person.phone[0].type_, "home");
        assert_eq!(person.phone[1].number, "+1800-867-5308");
        assert_eq!(person.phone[1].type_, "mobile");
    }
}
