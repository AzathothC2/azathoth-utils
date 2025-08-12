#[cfg(all(test, feature = "hasher"))]
mod hasher_tests {
    use azathoth_utils::crc32;
    use azathoth_utils::hasher::Hasher;
    #[test]
    fn hasher_closure_tuple() {
        let h: fn(&str) -> u32 = |name: &str| -> u32 { crc32(name.as_bytes())};
        let a = h.hash("LoadLibraryA");
        let b = h.hash("LoadLibraryA");
        assert_eq!(a, b);
    }
    
    #[test]
    fn hash_salted() {
        let salted = (|name: &str, salt: u32| -> u32 {
            crc32(name.as_bytes()) ^ salt
        }, 0xDEAD_BEEF);

        let s1 = salted.hash("GetProcAddress");
        let s2 = salted.hash("GetProcAddress");
        assert_eq!(s1, s2);
    }

    #[test]
    fn hash_garbage() {
        let h = |name: &str| -> u32 { crc32(name.as_bytes())};
        // Should not panic
        let garbage = [0xFF, 0xFE, 0xFD];
        let _ = h.hash_bytes(&garbage);
    }
}
