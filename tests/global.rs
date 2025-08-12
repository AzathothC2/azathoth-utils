#[cfg(test)]
mod global_tests {
    extern crate std;

    use azathoth_utils::crc32;

    #[test]
    fn crc32_test() {
        assert_eq!(crc32(b"123456789"), 0xCBF43926);
        assert_eq!(crc32(b"deadbeef"), 0x247F72D4);
    }

    #[test]
    fn crc32_test_empty() {
        assert_eq!(crc32(b""), 0xFFFFFFFFu32 ^ 0xFFFFFFFFu32);
    }

}