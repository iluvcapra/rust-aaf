#[allow(dead_code)]
#[allow(unused_imports)]

use uuid::Uuid;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub type OMByteOrder = u8;
pub type OMVersion = u8;
pub type OMPropertyCount = u16;
pub type OMPropertyId = u16;
pub type OMStoredForm = u16;
pub type OMPropertySize = u16;
pub type OMKeySize = u8;
pub type OMPropertyTag = u16;

pub type AAFUInt8 = u8;
pub type AAFUInt16 = u16;
pub type AAFUInt32 = u32;
pub type AAFUInt64 = u64;

pub type AAFInt8 = i8;
pub type AAFInt16 = i16;
pub type AAFInt32 = i32;
pub type AAFInt64 = i64;

pub type PositionType = AAFInt64;
pub type LengthType = AAFInt64;
pub type JPEGTableIDType = AAFInt32;
pub type PhaseFrameType = AAFInt32;

#[derive(Debug, PartialEq)]
pub struct TimeStamp {
    pub date: (i16, u8, u8),
    pub time: (u8, u8, u8, u8)
}

#[derive(Debug, PartialEq)]
pub struct VersionType {
    pub major: u8,
    pub minor: u8
}

pub trait AAFFrom {
    fn aaf_from(item: &[u8]) -> Self;
}

pub trait AAFInto<F> {
    fn aaf_into(self: Self) -> F;
}

impl<T> AAFInto<T> for &[u8] where T: AAFFrom {
    fn aaf_into(self: Self) -> T {
        T::aaf_from(self)
    }
}

impl AAFFrom for AAFUInt16 {
    fn aaf_from(item: &[u8]) -> Self {
        Cursor::new(item).read_u16::<LittleEndian>()
            .expect("Error reading AAFUInt16")
    }
}

impl AAFFrom for u32 {
    fn aaf_from(item: &[u8]) -> Self {
        Cursor::new(item).read_u32::<LittleEndian>()
            .expect("Failed to decode u32")
    }
}


impl AAFFrom for AAFUInt64 {
    fn aaf_from(item: &[u8]) -> Self {
        Cursor::new(item).read_u64::<LittleEndian>()
            .expect("Failed to decode AAFUInt64")
    }
}

impl AAFFrom for AAFInt16 {
    fn aaf_from(item: &[u8]) -> Self {
        Cursor::new(item).read_i16::<LittleEndian>()
            .expect("Error  reading AAFInt16")
    }
}

// impl TryFrom<&[u8]> for AAFInt32 {
//     type Error = io::Error;

//     fn try_from(item: &[u8]) -> io::Result<Self> {
//         Cursor::new(item).read_i32::<LittleEndian>()
//     }
// }

// impl TryFrom<&[u8]> for AAFInt64 {
//     type Error = io::Error;

//     fn try_from(item: &[u8]) -> io::Result<Self> {
//         Cursor::new(item).read_i64::<LittleEndian>()
//     }
// }

impl AAFFrom for TimeStamp {
    fn aaf_from(item: &[u8]) -> Self {
        if item.len() < 8 {
            panic!("TimeStamp record insufficient length")
        } else {
            TimeStamp {
                date: (item[0..2].aaf_into(), item[2].into(), item[3].into()),
                time: (item[4].into(), item[5].into(), item[6].into(), item[7].into())
            }
        }
    }
}

impl AAFFrom for VersionType {
    fn aaf_from(item: &[u8]) -> Self {
        if item.len() < 2 {
            panic!("VersionType record insufficient length")
        } else {
            VersionType {
                major: item[0].into(),
                minor: item[1].into()
            }
        }
    }
}

impl AAFFrom for Uuid {
    fn aaf_from(item: &[u8]) -> Self {
        if item.len() < 16 {
            panic!("Uuid record insufficient length")
        } else {
            let d1: AAFUInt32 = item[0..3].aaf_into();
            let d2: AAFUInt16 = item[3..5].aaf_into();
            let d3: AAFUInt16 = item[5..7].aaf_into();
            let d4: &[u8] = &item[7..15];
            Uuid::from_fields(d1, d2, d3, d4).unwrap()
        }
    }
}
