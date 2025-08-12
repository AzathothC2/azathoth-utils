use crate::errors::{AzUtilErrorCode, AzUtilResult};

/// A trait for types that can be searched for within a byte slice
pub trait Pattern {
    /// Returns true if the pattern matches the given window of bytes
    fn matches(&self, window: &[u8]) -> bool;

    /// Returns the length of the pattern
    fn len(&self) -> usize;

    /// Checks if the pattern is empty (has a length of 0)
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A basic byte pattern
///
/// # Example
/// ```
/// use azathoth_utils::psearch::{BasePattern, Searcher};
///
/// fn main() {
///     let memory_region = b"deadbeef_and_more_deadbeef_and_the_final_deadbeef";
///     let bpattern = BasePattern::new(b"deadbeef");
///     let mut bsearcher = Searcher::new(bpattern).unwrap();
///     let offsets: Vec<_> = bsearcher.search_all(memory_region).collect();
///     println!("Found offsets: {:?}", offsets);
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct BasePattern<'a> {
    pattern: &'a [u8],
}

impl<'a> BasePattern<'a> {
    /// Creates a new basic pattern object
    pub fn new(pattern: &'a [u8]) -> Self {
        Self { pattern }
    }
}
impl<'a> Pattern for BasePattern<'a> {
    #[inline(always)]
    fn matches(&self, window: &[u8]) -> bool {
        self.pattern == window
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.pattern.len()
    }
}

/// A byte pattern with a wildcard mask
/// In the mask, `1` means the byte must match, and `0` means it's a wildcard.
///
/// # Example
/// ```
/// use azathoth_utils::psearch::{MaskedPattern, Searcher};
///
/// fn main() {
///     let memory_region = b"deadbeef_and_more_deadbeef_and_the_final_deadbeef";
///     let mpattern = MaskedPattern::new(b"deadbeef", &[1, 0, 0, 1, 0, 0, 0, 1]).unwrap();
///     let mut msearcher = Searcher::new(mpattern).unwrap();
///     let offsets: Vec<_> = msearcher.search_all(memory_region).collect();
///     println!("Found offsets: {:?}", offsets);
/// }
/// ```
pub struct MaskedPattern<'a> {
    pattern: &'a [u8],
    mask: &'a [u8],
}
impl<'a> MaskedPattern<'a> {
    /// Creates a new masked pattern object
    pub fn new(pattern: &'a [u8], mask: &'a [u8]) -> AzUtilResult<Self> {
        if pattern.len() != mask.len() {
            return Err(AzUtilErrorCode::HashError);
        }
        Ok(Self { pattern, mask })
    }
}

impl<'a> Pattern for MaskedPattern<'a> {
    fn matches(&self, window: &[u8]) -> bool {
        self.pattern
            .iter()
            .zip(self.mask.iter())
            .zip(window.iter())
            .all(|((&p, &m), &w)| m == 0 || p == w)
    }
    fn len(&self) -> usize {
        self.pattern.len()
    }
}

/// A Generic searcher for any type that implements the [`Pattern`] trait
#[derive(Default, Debug)]
pub struct Searcher<P: Pattern> {
    pattern: P,
    last_result: Option<usize>,
}

impl<P: Pattern> Searcher<P> {
    /// Creates a new pattern searcher object
    pub fn new(pattern: P) -> AzUtilResult<Self> {
        if pattern.is_empty() {
            return Err(AzUtilErrorCode::HashError);
        }
        Ok(Self {
            pattern,
            last_result: None,
        })
    }

    /// Searches a given memory region for the first occurance of the pattern
    pub fn search(&mut self, region: &[u8]) -> Option<usize> {
        if region.len() < self.pattern.len() {
            return None;
        }
        let result = region
            .windows(self.pattern.len())
            .position(|window| self.pattern.matches(window));
        self.last_result = result;
        result
    }

    /// Returns an iterator over all non-overlapping matches of the pattern in the region.
    pub fn search_all<'searcher, 'region>(&'searcher mut self, region: &'region [u8]) -> SearchAll<'searcher, 'region, P> {
        SearchAll {
            searcher: self,
            region,
            current_offset: 0
        }
    }

    /// Gets the last found result, if any.
    #[inline(always)]
    pub fn result(&self) -> Option<usize> {
        self.last_result
    }

    /// Resets the context result for reuse with the same pattern.
    #[inline(always)]
    pub fn reset(&mut self) {
        self.last_result = None;
    }

    /// Returns a reference to the underlying pattern.
    pub fn pattern(&self) -> &P {
        &self.pattern
    }

    /// Resets the searcher and sets a new pattern
    pub fn set_pattern(&mut self, pattern: P) {
        self.pattern = pattern;
        self.reset();
    }
}

/// Iterator over all matches in a region. Produced by [`Searcher::search_all`]
pub struct SearchAll<'searcher, 'region, P: Pattern> {
    searcher: &'searcher mut Searcher<P>,
    region: &'region [u8],
    current_offset: usize
}

impl<'searcher, 'region, P: Pattern> Iterator for SearchAll<'searcher, 'region, P> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_offset + self.searcher.pattern.len() > self.region.len() {
            return None;
        }

        let remaining_region = &self.region[self.current_offset..];
        match self.searcher.search(remaining_region) {
            Some(found) => {
                let pos = self.current_offset + found;
                self.current_offset += found + 1;
                Some(pos)
            }
            None => {
                self.current_offset += self.region.len();
                None
            }
        }
    }
}