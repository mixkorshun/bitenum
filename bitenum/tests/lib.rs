use bitenum::{BitEnum, BitEnumSet};

enum TestBitEnum {
    A = 1 << 0,
    B = 1 << 1,
}

impl Into<u32> for TestBitEnum {
    fn into(self) -> u32 { self as u32 }
}

impl BitEnum for TestBitEnum {
    type Scalar = u32;
}

#[test]
fn test_mutable_ops() {
    let mut set = BitEnumSet::new();

    assert_eq!(set.contains(TestBitEnum::A), false);

    set.insert(TestBitEnum::A);

    assert_eq!(set.contains(TestBitEnum::A), true);
    assert_eq!(set.contains(TestBitEnum::B), false);

    set.remove(TestBitEnum::A);

    assert_eq!(set.contains(TestBitEnum::A), false);
}

#[test]
fn test_immutable_ops() {
    let set = BitEnumSet::new();

    assert_eq!(set.contains(TestBitEnum::A), false);

    let set = set.with(TestBitEnum::A);
    assert_eq!(set.contains(TestBitEnum::A), true);
    assert_eq!(set.contains(TestBitEnum::B), false);

    let set = set.without(TestBitEnum::A);
    assert_eq!(set.contains(TestBitEnum::A), false);
}