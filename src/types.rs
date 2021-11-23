#[allow(dead_code)]
#[allow(unused_imports)]

use uuid::Uuid;
use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt};

pub type OMByteOrder = u8;
pub type OMVersion = u8;
pub type OMPropertyCount = u16;
pub type OMPropertyId = u16;
pub type OMStoredForm = u16;
pub type OMPropertySize = u16;
pub type OMKeySize = u8;
pub type OMPropertyTag = u16;

trait AAFType { 
    fn from_data(data: &[u8]) -> Self;
    fn into_data(&self, data: &mut [u8]);
}

type AAFUInt8 = u8;
type AAFUInt16 = u16;
type AAFUInt32 = u32;
type AAFUInt64 = u64;

type AAFInt8 = i8;
type AAFInt16 = i16;
type AAFInt32 = i32;
type AAFInt64 = i64;

type PositionType = AAFInt64;
type LengthType = AAFInt64;
type JPEGTableIDType = AAFInt32;
type PhaseFrameType = AAFInt32;


impl AAFType for AAFUInt8 {
    
    fn from_data(data: &[u8]) -> Self {
        let mut c = Cursor::new(data);
        c.read_u8().unwrap()
    }

    fn into_data(&self, data: &mut [u8]) {
        let mut c = Cursor::new(data);
        c.write_u8(*self).expect("Failed to write u8 data into AAFUInt8")
    }
}

