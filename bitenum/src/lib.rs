#![no_std]

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

pub trait BitEnum: Into<Self::Scalar> {
    type Scalar: BitAnd<Output=Self::Scalar> + BitOr<Output=Self::Scalar> + Not<Output=Self::Scalar> + From<u8> + PartialEq + Copy;
}

pub struct BitEnumSet<UInt, Enum: BitEnum<Scalar=UInt>>(pub Enum::Scalar);

impl<UInt, Enum> BitEnumSet<UInt, Enum>
    where UInt: BitAnd<Output=UInt> + BitAndAssign + BitOr<Output=UInt> + BitOrAssign + Not<Output=UInt> + PartialEq + From<u8> + Copy,
          Enum: BitEnum<Scalar=UInt> {
    #[inline]
    pub fn new() -> Self { Self(UInt::from(0)) }

    pub fn contains(&self, value: Enum) -> bool {
        return (self.0 & value.into()) != UInt::from(0);
    }

    pub fn insert(&mut self, value: Enum) -> &Self {
        self.0 |= value.into();
        self
    }

    pub fn remove(&mut self, value: Enum) -> &Self {
        self.0 &= !value.into();
        self
    }

    #[inline]
    pub fn with(&self, value: Enum) -> Self {
        Self(self.0 | value.into())
    }

    #[inline]
    pub fn without(&self, value: Enum) -> Self {
        Self(self.0 & !value.into())
    }
}
