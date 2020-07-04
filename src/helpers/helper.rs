pub fn bit_set(value: u8, location: u8) -> bool {
    ((value >> location) & 1) == 1
}

#[test]
fn test_bit_set() {
    assert_eq!(true, bit_set(1, 0));
    assert_eq!(true, bit_set(2, 1));
    assert_eq!(true, bit_set(4, 2));
    assert_eq!(true, bit_set(8, 3));
    assert_eq!(true, bit_set(16, 4));
    assert_eq!(true, bit_set(32, 5));
    assert_eq!(true, bit_set(64, 6));
    assert_eq!(true, bit_set(128, 7));
    assert_eq!(true, bit_set(255, 7));

    assert_eq!(false, bit_set(254, 0));
    assert_eq!(false, bit_set(253, 1));
    assert_eq!(false, bit_set(64, 3));
}
