/// Represents different forms of a function identifier that can be used for hashing.
///
/// This enum allows identifying a function either by:
/// - A precomputed 32-bit hash value (`Hashed`)
/// - A string slice containing the function name (`Name`)
/// - A raw byte slice (`Bytes`)
///
/// Lifetime `'a` is used to ensure that borrowed data (`&str`, `&[u8]`) outlives the identifier.
pub enum FuncIdentifier<'a> {
    /// A precomputed 32-bit hash value.
    Hashed(u32),
    /// A borrowed string slice representing the function name.
    Name(&'a str),
    /// A borrowed byte slice representing the function name.
    Bytes(&'a [u8]),
}


impl<'a> From<u32> for FuncIdentifier<'a> {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self::Hashed(value)
    }
}

impl<'a> From<&'a str> for FuncIdentifier<'a> {
    fn from(value: &'a str) -> Self {
        Self::Name(value)
    }
}

impl<'a> From<&'a [u8]> for FuncIdentifier<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::Bytes(value)
    }
}

/// A trait that defines a hashing interface for converting function identifiers (strings or bytes)
/// into a 32-bit hash value.
///
/// Types implementing this trait can be used as hashing strategies for function name resolution.
pub trait Hasher {
    /// Hashes a function name (string slice) into a 32-bit hash value.
    ///
    /// # Returns
    /// A 32-bit hash of the input name.
    fn hash(&self, name: &str) -> u32;
    /// Hashes a byte slice into a 32-bit hash value.
    ///
    /// This is a default implementation that attempts to convert the byte slice to a UTF-8 string.
    /// If successful, it delegates to the [`hash`] function.
    /// If not valid UTF-8, it performs a fallback rotation-based hash on raw bytes.
    ///
    /// # Returns
    /// A 32-bit hash of the input bytes.
    fn hash_bytes(&self, bytes: &[u8]) -> u32 {
        match core::str::from_utf8(bytes) {
            Ok(s) => self.hash(s),
            Err(_) => {
                let mut val = 0u32;
                for &b in bytes {
                    val = val.wrapping_add(b as u32).rotate_left(5);
                }
                val
            }
        }
    }
}

impl<F> Hasher for F
where
    F: for<'a> Fn(&'a str) -> u32,
{
    #[inline(always)]
    fn hash(&self, name: &str) -> u32 {
        self(name)
    }
}


macro_rules! impl_hasher_tuple {
    ($( $($T:ident),+ );+ $(;)?) => {
        $(
            #[allow(non_snake_case)]
            impl<F, $($T),+> Hasher for (F, $($T),+)
            where
                F: Fn(&str, $($T),+) -> u32,
                $( $T: Copy ),+
            {
                #[inline(always)]
                fn hash(&self, name: &str) -> u32 {
                    let (f, $($T),+) = self;
                    (f)(name, $( *$T ),+)
                }
            }
        )+
    }
}

impl_hasher_tuple!(
    A;
    A, B;
    A, B, C;
    A, B, C, D
);
