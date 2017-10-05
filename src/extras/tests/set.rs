use extras::Set;

#[test]
fn test_blank() {
    let extra = Set::new();
    let raw: &[u8] = extra.as_ref();
    assert_eq!(0, extra.flags());
    assert_eq!(0, extra.expiration());
    assert_eq!([0; 8], raw);
}

#[test]
fn test_flags() {
    let extra = Set::new().with_flags(0xdeadbeef_u32);
    let raw: &[u8] = extra.as_ref();
    assert_eq!(0xdeadbeef, extra.flags());
    assert_eq!(0, extra.expiration());
    assert_eq!([0xde, 0xad, 0xbe, 0xef], &raw[0..4]);
    assert_eq!([0x00, 0x00, 0x00, 0x00], &raw[4..8]);
}

#[test]
fn test_expiration() {
    let extra = Set::new().with_expiration(360u32);
    let raw: &[u8] = extra.as_ref();
    assert_eq!(0, extra.flags());
    assert_eq!(360, extra.expiration());
    assert_eq!([0x00, 0x00, 0x00, 0x00], &raw[0..4]);
    assert_eq!([0x00, 0x00, 0x01, 0x68], &raw[4..8]);
}
