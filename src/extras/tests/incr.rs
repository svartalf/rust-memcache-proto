use extras::Increment;

#[test]
fn test_blank() {
    let extra = Increment::default();
    let raw: &[u8] = extra.as_ref();
    assert_eq!(0, extra.amount());
    assert_eq!([0; 8], &raw[0..8]);
    assert_eq!(0, extra.initial());
    assert_eq!([0; 8], &raw[8..16]);
    assert_eq!(0, extra.expiration());
    assert_eq!([0; 4], &raw[16..20]);
}

#[test]
fn test_flags() {
    let extra = Increment::build().amount(1).finish();
    let raw: &[u8] = extra.as_ref();
    assert_eq!(1, extra.amount());
    assert_eq!([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01], &raw[0..8]);
    assert_eq!(0, extra.initial());
    assert_eq!([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], &raw[8..16]);
    assert_eq!(0, extra.expiration());
    assert_eq!([0x00, 0x00, 0x00, 0x00], &raw[16..20]);
}

#[test]
fn test_initial() {
    let extra = Increment::build().initial(42).finish();
    let raw: &[u8] = extra.as_ref();
    assert_eq!(0, extra.amount());
    assert_eq!([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], &raw[0..8]);
    assert_eq!(42, extra.initial());
    assert_eq!([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a], &raw[8..16]);
    assert_eq!(0, extra.expiration());
    assert_eq!([0x00, 0x00, 0x00, 0x00], &raw[16..20]);
}

#[test]
fn test_expiration() {
    let extra = Increment::build().expiration(360).finish();
    let raw: &[u8] = extra.as_ref();
    assert_eq!(0, extra.amount());
    assert_eq!([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], &raw[0..8]);
    assert_eq!(0, extra.initial());
    assert_eq!([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], &raw[8..16]);
    assert_eq!(360, extra.expiration());
    assert_eq!([0x00, 0x00, 0x01, 0x68], &raw[16..20]);
}
