#[macro_export]
macro_rules! set_bit {
    ($byte:expr, $bit:expr) => {
        $byte | (1 << ($bit - 1))
    };
}

#[macro_export]
macro_rules! clr_bit {
    ($byte:expr, $bit:expr) => {
        $byte & (!(1 << ($bit - 1)))
    };
}

#[macro_export]
macro_rules! tgl_bit {
    ($byte:expr, $bit:expr) => {
        $byte ^ (1 << ($bit - 1))
    };
}

#[macro_export]
macro_rules! get_bit {
    ($byte:expr, $bit:expr) => {
        $byte >> ($bit - 1) & 1
    };
}

pub fn word_split(value: u16) -> (u8, u8) {
    return ((value >> 8) as u8, value as u8);
}

pub fn dword_split(value: u32) -> (u16, u16) {
    return ((value >> 16) as u16, value as u16);
}

pub trait BitSel<Rhs = Self> {
    type Output;
    fn bitsel(self, rhs: Rhs) -> Self::Output;
}

macro_rules! bitsel_impl {
    ($($t:ty)*) => ($(
        impl BitSel for $t {
            type Output = $t;

            #[inline]
            fn bitsel(self, rhs: $t) -> $t { get_bit!(self, rhs) }
        }
    )*)
}

bitsel_impl! {usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128}

pub trait Increment<Rhs = Self> {
    type Output;
    fn increment(self) -> Self::Output;
}

macro_rules! increment_impl {
    ($($t:ty)*) => ($(
        impl Increment for $t {
            type Output = $t;

            #[inline]
            fn increment(self) -> $t { self + 1 }
        }
    )*)
}

increment_impl! {usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128}

pub trait Decrement<Rhs = Self> {
    type Output;
    fn decrement(self) -> Self::Output;
}

macro_rules! decrement_impl {
    ($($t:ty)*) => ($(
        impl Decrement for $t {
            type Output = $t;

            #[inline]
            fn decrement(self) -> $t { self - 1 }
        }
    )*)
}

decrement_impl! {usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128}

pub use clr_bit;
pub use set_bit;
pub use tgl_bit;
pub use get_bit;

#[cfg(test)]
mod tests {
    use super::{clr_bit, set_bit, get_bit, tgl_bit};

    #[test]
    fn bitwise_operation_correctness() {
        assert_eq!(clr_bit!(7, 2), 5);
        assert_eq!(clr_bit!(7, 1), 6);

        assert_eq!(get_bit!(6, 1), 0);
        assert_eq!(get_bit!(6, 2), 1);

        assert_eq!(set_bit!(5, 2), 7);
        assert_eq!(set_bit!(3, 4), 11);

        assert_eq!(tgl_bit!(7, 2), tgl_bit!(13, 4));
        assert_eq!(tgl_bit!(8, 4), 0);
    }
}