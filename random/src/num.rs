use std::ops::{Add, Rem, Sub};

pub trait Num: Sized + Copy + From<u8>
+ Add<Output = Self> + Sub<Output = Self> + Rem<Output = Self> + PartialOrd {
    fn from_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_byte_nums (( $($int:ident),* ) => {
    $(
        impl Num for $int {
            fn from_bytes(bytes: &[u8]) -> Self { Self::from_le_bytes(bytes.try_into().unwrap()) }
        }
    )*
});

impl_byte_nums!(u8, u16, u32, u64);