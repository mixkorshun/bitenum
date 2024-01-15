use bitenum_macro::bitenum;

#[bitenum(u32)]
enum TestBitEnum {
    A,
    B,
}

#[test]
fn test_mutable_ops() {
    use bitenum::BitEnumSet;

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
    use bitenum::BitEnumSet;

    let set = BitEnumSet::new();

    assert_eq!(set.contains(TestBitEnum::A), false);

    let set = set.with(TestBitEnum::A);
    assert_eq!(set.contains(TestBitEnum::A), true);
    assert_eq!(set.contains(TestBitEnum::B), false);

    let set = set.without(TestBitEnum::A);
    assert_eq!(set.contains(TestBitEnum::A), false);
}