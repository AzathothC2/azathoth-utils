#[cfg(all(feature = "psearch", test))]
mod psearch_tests {
    use azathoth_utils::psearch::{BasePattern, MaskedPattern, Searcher};

    #[test]
    fn find_basic_pattern() {
        let mut s = Searcher::new(BasePattern::new(b"deadbeef")).expect("pattern ok");
        let hay = b"xxxdeadbeefyyy";
        let pos = s.search(hay).expect("search returned None");
        assert_eq!(pos, 3);
    }

    #[test]
    fn find_masked_pattern() {
        let mpattern = MaskedPattern::new(b"deadbeef", &[1, 0, 0, 1, 0, 0, 0, 1]).unwrap();
        let mut msearcher = Searcher::new(mpattern).unwrap();
        let hay = b"deadbeef_and_more_deadbeef_and_the_final_deadbeef";
        let pos = msearcher.search(hay).expect("search returned None");
        assert_eq!(pos, 0);
    }

    #[test]
    fn return_none_on_empty_result() {
        let mut s = Searcher::new(BasePattern::new(b"abc")).expect("pattern ok");
        let hay = b"xyz";
        assert!(s.search(hay).is_none());
    }
}
