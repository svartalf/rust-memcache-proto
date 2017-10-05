use extras::Get;

#[test]
fn test_blank() {
    let extra = Get::new();
    let raw: &[u8] = extra.as_ref();
    assert_eq!(0, extra.flags());
    assert_eq!([0; 4], raw);
}

#[test]
fn test_flags() {
    let extra = Get::new().with_flags(0xdeadbeef);
    let raw: &[u8] = extra.as_ref();
    assert_eq!(0xdeadbeef, extra.flags());
    assert_eq!([0xde, 0xad, 0xbe, 0xef], raw);
}