// use std::default;

pub enum EndianByteOrdering {
    LittleEndian,
    BitEndian,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum DataWordSized {
    DataSizeByte(u8),
    DataSizeWord(u16),
    DataSizeDouble(u32),
    DataSizeLong(u64),
    DataSizeSignedByte(i8),
    DataSizeSignedWord(i16),
    DataSizeSignedDouble(i32),
    DataSizeSignedLong(i64),
    #[default]
    Invalid,
}

impl From<u8> for DataWordSized {
    fn from(value: u8) -> Self {
        DataWordSized::DataSizeByte(value)
    }
}

impl From<u16> for DataWordSized {
    fn from(value: u16) -> Self {
        DataWordSized::DataSizeWord(value)
    }
}

impl From<u32> for DataWordSized {
    fn from(value: u32) -> Self {
        DataWordSized::DataSizeDouble(value)
    }
}

impl From<u64> for DataWordSized {
    fn from(value: u64) -> Self {
        DataWordSized::DataSizeLong(value)
    }
}

impl From<i8> for DataWordSized {
    fn from(value: i8) -> Self {
        DataWordSized::DataSizeSignedByte(value)
    }
}

impl From<i16> for DataWordSized {
    fn from(value: i16) -> Self {
        DataWordSized::DataSizeSignedWord(value)
    }
}

impl From<i32> for DataWordSized {
    fn from(value: i32) -> Self {
        DataWordSized::DataSizeSignedDouble(value)
    }
}

impl From<i64> for DataWordSized {
    fn from(value: i64) -> Self {
        DataWordSized::DataSizeSignedLong(value)
    }
}

impl std::fmt::Display for DataWordSized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataWordSized::DataSizeByte(x) => write!(f, "{}", x),
            DataWordSized::DataSizeWord(x) => write!(f, "{}", x),
            DataWordSized::DataSizeDouble(x) => write!(f, "{}", x),
            DataWordSized::DataSizeLong(x) => write!(f, "{}", x),
            DataWordSized::DataSizeSignedByte(x) => write!(f, "{}", x),
            DataWordSized::DataSizeSignedWord(x) => write!(f, "{}", x),
            DataWordSized::DataSizeSignedDouble(x) => write!(f, "{}", x),
            DataWordSized::DataSizeSignedLong(x) => write!(f, "{}", x),
            DataWordSized::Invalid => write!(f, "#$?"),
        }
    }
}

impl Default for &DataWordSized {
    fn default() -> Self {
        &DataWordSized::Invalid
    }
}

macro_rules! from_data_word_sized_impl {
    ($($t:ty)*) => ($(
        impl From<DataWordSized> for $t {
            fn from(value: DataWordSized) -> Self {
                match value {
                    DataWordSized::DataSizeByte(x) => x as $t,
                    DataWordSized::DataSizeWord(x) => x as $t,
                    DataWordSized::DataSizeDouble(x) => x as $t,
                    DataWordSized::DataSizeLong(x) => x as $t,
                    DataWordSized::DataSizeSignedByte(x) => x as $t,
                    DataWordSized::DataSizeSignedWord(x) => x as $t,
                    DataWordSized::DataSizeSignedDouble(x) => x as $t,
                    DataWordSized::DataSizeSignedLong(x) => x as $t,
                    DataWordSized::Invalid => 0,
                }
            }
        }
    )*)
}

from_data_word_sized_impl! {u8 u16 u32 u64 i8 i16 i32 i64}

impl DataWordSized {
    pub fn as_u8(&self) -> u8 {
        (*self).into()
    }

    pub fn as_u16(&self) -> u16 {
        (*self).into()
    }

    pub fn as_u32(&self) -> u32 {
        (*self).into()
    }

    pub fn as_u64(&self) -> u64 {
        (*self).into()
    }
}

impl EndianByteOrdering {
    pub fn compose(&self, bytes: Vec<u8>) -> DataWordSized {
        return match (self, bytes.len()) {
            (EndianByteOrdering::BitEndian, 1) => bytes[0].into(),
            (EndianByteOrdering::BitEndian, 2) => {
                u16::from_be_bytes(bytes.as_slice().try_into().unwrap()).into()
            }
            (EndianByteOrdering::BitEndian, 4) => {
                u32::from_be_bytes(bytes.as_slice().try_into().unwrap()).into()
            }
            (EndianByteOrdering::BitEndian, 8) => {
                u64::from_be_bytes(bytes.as_slice().try_into().unwrap()).into()
            }
            (EndianByteOrdering::LittleEndian, 1) => bytes[0].into(),
            (EndianByteOrdering::LittleEndian, 2) => {
                u16::from_le_bytes(bytes.as_slice().try_into().unwrap()).into()
            }
            (EndianByteOrdering::LittleEndian, 4) => {
                u32::from_le_bytes(bytes.as_slice().try_into().unwrap()).into()
            }
            (EndianByteOrdering::LittleEndian, 8) => {
                u64::from_le_bytes(bytes.as_slice().try_into().unwrap()).into()
            }
            _ => DataWordSized::DataSizeByte(0),
        };
    }

    pub fn settle(&self, word: DataWordSized) -> Vec<u8> {
        match (word, self) {
            (DataWordSized::DataSizeByte(value), EndianByteOrdering::LittleEndian) => {
                value.to_le_bytes().to_vec()
            }
            (DataWordSized::DataSizeByte(value), EndianByteOrdering::BitEndian) => {
                value.to_be_bytes().to_vec()
            }
            (DataWordSized::DataSizeWord(value), EndianByteOrdering::LittleEndian) => {
                value.to_le_bytes().to_vec()
            }
            (DataWordSized::DataSizeWord(value), EndianByteOrdering::BitEndian) => {
                value.to_be_bytes().to_vec()
            }
            (DataWordSized::DataSizeDouble(value), EndianByteOrdering::LittleEndian) => {
                value.to_le_bytes().to_vec()
            }
            (DataWordSized::DataSizeDouble(value), EndianByteOrdering::BitEndian) => {
                value.to_be_bytes().to_vec()
            }
            (DataWordSized::DataSizeLong(value), EndianByteOrdering::LittleEndian) => {
                value.to_le_bytes().to_vec()
            }
            (DataWordSized::DataSizeLong(value), EndianByteOrdering::BitEndian) => {
                value.to_be_bytes().to_vec()
            }
            _ => vec![0],
        }
    }
}
