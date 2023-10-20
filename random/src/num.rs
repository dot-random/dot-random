use std::ops::{Add, BitAnd, Rem, Sub};

pub trait Num: Sized + Copy + From<u8> + BitAnd<Output = Self>
+ Add<Output = Self> + Sub<Output = Self> + Rem<Output = Self> + PartialOrd {
    const MAX: Self;
    fn from_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_byte_nums (( $($int:ident),* ) => {
    $(
        impl Num for $int {
            const MAX: $int = $int::MAX;
            fn from_bytes(bytes: &[u8]) -> Self { Self::from_be_bytes(bytes.try_into().unwrap()) }
        }
    )*
});

impl_byte_nums!(u8, u16, u32, u64, usize);