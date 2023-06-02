pub enum EndianByteOrdering {
    LittleEndian,
    BitEndian,
}


#[derive(Debug)]
pub enum DataWordSized {
    DataSizeByte(u8),
    DataSizeWord(u16),
    DataSizeDouble(u32),
    DataSizeLong(u64),
    Invalid,
}

impl DataWordSized {

    pub fn as_u8(&self) -> u8 {
        return match *self {
            Self::Invalid => 0,
            Self::DataSizeByte(x) => x,
            Self::DataSizeWord(x) => x as u8,
            Self::DataSizeDouble(x) => x as u8,
            Self::DataSizeLong(x) => x as u8
        }
    }


    pub fn as_u16(&self) -> u16 {
        return match *self {
            Self::Invalid => 0,
            Self::DataSizeByte(x) => x as u16,
            Self::DataSizeWord(x) => x,
            Self::DataSizeDouble(x) => x as u16,
            Self::DataSizeLong(x) => x as u16
        }
    }

    pub fn as_u32(&self) -> u32 {
        return match *self {
            Self::Invalid => 0,
            Self::DataSizeByte(x) => x as u32,
            Self::DataSizeWord(x) => x as u32,
            Self::DataSizeDouble(x) => x,
            Self::DataSizeLong(x) => x as u32
        }
    }

    pub fn as_u64(&self) -> u64 {
        return match *self {
            Self::Invalid => 0,
            Self::DataSizeByte(x) => x as u64,
            Self::DataSizeWord(x) => x as u64,
            Self::DataSizeDouble(x) => x as u64,
            Self::DataSizeLong(x) => x
        }
    }

}


impl EndianByteOrdering {
    pub fn compose(&self, bytes: Vec<u8>) -> DataWordSized {
        return match (self, bytes.len()) {
            (EndianByteOrdering::BitEndian, 1) => DataWordSized::DataSizeByte(bytes[0]),
            (EndianByteOrdering::BitEndian, 2) => DataWordSized::DataSizeWord(u16::from_be_bytes(bytes.as_slice().try_into().unwrap())),
            (EndianByteOrdering::BitEndian, 4) => DataWordSized::DataSizeDouble(u32::from_be_bytes(bytes.as_slice().try_into().unwrap())),
            (EndianByteOrdering::BitEndian, 8) => DataWordSized::DataSizeLong(u64::from_be_bytes(bytes.as_slice().try_into().unwrap())),
            (EndianByteOrdering::LittleEndian, 1) => DataWordSized::DataSizeByte(bytes[0]),
            (EndianByteOrdering::LittleEndian, 2) => DataWordSized::DataSizeWord(u16::from_le_bytes(bytes.as_slice().try_into().unwrap())),
            (EndianByteOrdering::LittleEndian, 4) => DataWordSized::DataSizeDouble(u32::from_le_bytes(bytes.as_slice().try_into().unwrap())),
            (EndianByteOrdering::LittleEndian, 8) => DataWordSized::DataSizeLong(u64::from_le_bytes(bytes.as_slice().try_into().unwrap())),
            _ => DataWordSized::DataSizeByte(0),
        };
    }

    pub fn settle(&self, word: DataWordSized) -> Vec<u8> {
        match (word, self) {
            (DataWordSized::DataSizeByte(value), EndianByteOrdering::LittleEndian) => value.to_le_bytes().to_vec(),
            (DataWordSized::DataSizeByte(value), EndianByteOrdering::BitEndian) => value.to_be_bytes().to_vec(),
            (DataWordSized::DataSizeWord(value), EndianByteOrdering::LittleEndian) => value.to_le_bytes().to_vec(),
            (DataWordSized::DataSizeWord(value), EndianByteOrdering::BitEndian) => value.to_be_bytes().to_vec(),
            (DataWordSized::DataSizeDouble(value), EndianByteOrdering::LittleEndian) => value.to_le_bytes().to_vec(),
            (DataWordSized::DataSizeDouble(value), EndianByteOrdering::BitEndian) => value.to_be_bytes().to_vec(),
            (DataWordSized::DataSizeLong(value), EndianByteOrdering::LittleEndian) => value.to_le_bytes().to_vec(),
            (DataWordSized::DataSizeLong(value), EndianByteOrdering::BitEndian) => value.to_be_bytes().to_vec(),
            (DataWordSized::Invalid, EndianByteOrdering::LittleEndian) => vec![0],
            (DataWordSized::Invalid, EndianByteOrdering::BitEndian) => vec![0],
        }
    }
}
