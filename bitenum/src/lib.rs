#![no_std]

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

pub use bitenum_macro::bitenum;

pub trait BitEnum: Into<Self::Scalar> {
    type Scalar: BitAnd<Output=Self::Scalar> + BitOr<Output=Self::Scalar> + Not<Output=Self::Scalar> + From<u8> + PartialEq + Copy;
}

pub struct BitEnumSet<Enum: BitEnum>(Enum::Scalar);

impl<Scalar, Enum> BitEnumSet<Enum>
    where Scalar: BitAnd<Output=Scalar> + BitAndAssign + BitOr<Output=Scalar> + BitOrAssign + Not<Output=Scalar> + PartialEq + From<u8> + Copy,
          Enum: BitEnum<Scalar=Scalar> {
    #[inline]
    pub fn new() -> Self { Self(Enum::Scalar::from(0)) }

    pub fn contains(&self, value: Enum) -> bool {
        return (self.0 & value.into()) != Scalar::from(0);
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
