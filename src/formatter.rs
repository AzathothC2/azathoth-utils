use super::errors::{AzUtilResult, AzUtilErrorCode};
use alloc::string::String;
use alloc::vec::Vec;
use core::str;

/// Minimal writable buffer trait
pub trait WriteBuffer {
    /// Writes a `&str` into the buffer
    fn write_str(&mut self, s: &str) -> AzUtilResult<()>;
}

/// Minimal string type
///
/// This type exists to make sure there are no surprises during the format operation
pub struct AllocString {
    buf: Vec<u8>,
}

impl AllocString {
    /// Creates a new `AllocString`
    pub fn new() -> AllocString {
        Self { buf: Vec::new() }
    }

    /// Creates a new [`AllocString`] with a specified capacity
    pub fn with_capacity(capacity: usize) -> AllocString {
        Self {
            buf: Vec::with_capacity(capacity),
        }
    }

    /// Converts the [`AllocString`] into an [`String`] type
    pub fn into_string(self) -> AzUtilResult<String> {
        String::from_utf8(self.buf).map_err(|_| AzUtilErrorCode::FormatError)
    }

    /// Extend the current [`AllocString`] with an additional `&str`
    pub fn push_str(&mut self, s: &str) -> AzUtilResult<()> {
        self.buf.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

impl WriteBuffer for AllocString {
    fn write_str(&mut self, s: &str) -> AzUtilResult<()> {
        self.push_str(s)
    }
}

impl WriteBuffer for String {
    fn write_str(&mut self, s: &str) -> AzUtilResult<()> {
        self.extend(s.chars());
        Ok(())
    }
}

impl WriteBuffer for Vec<u8> {
    fn write_str(&mut self, s: &str) -> AzUtilResult<()> {
        self.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

/// Format specifier struct
///
/// Used to track the format specifiers in a string
#[derive(Default, Debug)]
pub struct FormatSpec {
    alternate: bool,
    specifier: char,
}

impl FormatSpec {
    /// Parses a string for extended format specifiers
    ///
    /// This function may be extended in the future, but for now it only searches for `:` and `#` chars
    pub fn parse_spec(s: &str) -> Self {
        let mut spec = FormatSpec::default();
        if s.is_empty() {
            return spec;
        }
        let mut chars = s.chars();
        if s.starts_with(':') {
            chars.next();
        }
        if chars.as_str().starts_with('#') {
            spec.alternate = true;
            chars.next();
        }
        if let Some(c) = chars.last() {
            spec.specifier = c;
        }
        spec
    }
}
/// Custom formatter replacement for the [`core::fmt::Display`] trait
pub trait FDisplay {
    /// The caller must implement this function to use the [`crate::format_str!`] macro
    fn fmt<W: WriteBuffer>(&self, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()>;
}

fn u64_to_str_decimal<W: WriteBuffer>(mut n: u64, buf: &mut W) -> AzUtilResult<()> {
    if n == 0 {
        return buf.write_str("0");
    }
    let mut temp_buf = [0u8; 20];
    let mut i = 0;
    while n > 0 {
        temp_buf[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }
    let digits = &mut temp_buf[..i];
    digits.reverse();
    buf.write_str(unsafe { str::from_utf8_unchecked(digits) })
}

fn u64_to_str_radix<W: WriteBuffer>(
    mut n: u64,
    radix: u32,
    uppercase: bool,
    buf: &mut W,
) -> AzUtilResult<()> {
    if n == 0 {
        return buf.write_str("0");
    }
    let mut temp_buf = [0u8; 64];
    let mut i = 0;
    let charset = if uppercase {
        b"0123456789ABCDEF"
    } else {
        b"0123456789abcdef"
    };
    while n > 0 {
        temp_buf[i] = charset[(n % (radix as u64)) as usize];
        n /= radix as u64;
        i += 1;
    }
    let digits = &mut temp_buf[..i];
    digits.reverse();
    buf.write_str(unsafe { str::from_utf8_unchecked(digits) })
}

impl<'a, T: ?Sized + FDisplay> FDisplay for &'a T {
    fn fmt<W: WriteBuffer>(&self, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()> {
        (*self).fmt(w, spec)
    }
}
impl FDisplay for str {
    fn fmt<W: WriteBuffer>(&self, w: &mut W, _spec: &FormatSpec) -> AzUtilResult<()> {
        w.write_str(self)
    }
}
impl FDisplay for String {
    fn fmt<W: WriteBuffer>(&self, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()> {
        self.as_str().fmt(w, spec)
    }
}
impl FDisplay for char {
    fn fmt<W: WriteBuffer>(&self, w: &mut W, _spec: &FormatSpec) -> AzUtilResult<()> {
        let mut buffer = [0u8; 4];
        w.write_str(self.encode_utf8(&mut buffer))
    }
}
impl FDisplay for bool {
    fn fmt<W: WriteBuffer>(&self, w: &mut W, _spec: &FormatSpec) -> AzUtilResult<()> {
        w.write_str(if *self { "true" } else { "false" })
    }
}
impl<T: FDisplay> FDisplay for Option<T> {
    fn fmt<W: WriteBuffer>(&self, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()> {
        match self {
            Some(v) => v.fmt(w, spec),
            None => w.write_str("None"),
        }
    }
}
impl<T: FDisplay, E: FDisplay> FDisplay for Result<T, E> {
    fn fmt<W: WriteBuffer>(&self, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()> {
        match self {
            Ok(v) => v.fmt(w, spec),
            Err(e) => {
                w.write_str("Err(")?;
                e.fmt(w, spec)?;
                w.write_str(")")
            }
        }
    }
}

impl<T: FDisplay> FDisplay for Vec<T> {
    fn fmt<W: WriteBuffer>(&self, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()> {
        w.write_str("[")?;
        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                w.write_str(", ")?;
            }
            item.fmt(w, spec)?;
        }
        w.write_str("]")
    }
}

impl<T> FDisplay for *const T {
    fn fmt<W: WriteBuffer>(&self, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()> {
        if spec.specifier != 'p' && spec.specifier != '\0' {
            return Err(AzUtilErrorCode::FormatError);
        }
        w.write_str("0x")?;
        u64_to_str_radix(*self as usize as u64, 16, false, w)
    }
}
impl<T> FDisplay for *mut T {
    fn fmt<W: WriteBuffer>(&self, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()> {
        (*self as *const T).fmt(w, spec)
    }
}

macro_rules! impl_display_uint {
    ($($t:ty),*) => {
        $(impl FDisplay for $t { fn fmt<W: WriteBuffer>(&self, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()> {
            let val = *self as u64;
            fmt_spec(val, w, spec)
        } })*
    };
}
macro_rules! impl_display_int {
    ($($t:ty),*) => {
        $(impl FDisplay for $t { fn fmt<W: WriteBuffer>(&self, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()> {
            let is_negative = *self < 0;
            let val = self.unsigned_abs() as u64;
            if is_negative { w.write_str("-")?; }
            fmt_spec(val, w, spec)
        } })*
    };
}
impl_display_uint!(u8, u16, u32, u64, u128, usize);
impl_display_int!(i8, i16, i32, i64, i128, isize);

fn fmt_spec<W: WriteBuffer>(val: u64, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()> {
    match spec.specifier {
        'x' | 'X' => {
            if spec.alternate {
                w.write_str("0x")?;
            }
            u64_to_str_radix(val, 16, spec.specifier == 'X', w)
        }
        'b' => {
            if spec.alternate {
                w.write_str("0b")?;
            }
            u64_to_str_radix(val, 2, false, w)
        }
        _ => u64_to_str_decimal(val, w),
    }
}

/// Argument formatting trait
///
/// The trait requires each implementor to implement the [`FormatArgs::format_at`] function
pub trait FormatArgs {
    /// Argument formatting function
    ///
    /// Expects a writable buffer that implements the [`WriteBuffer`] trait, an index for specifying where to place the formatted value,
    /// and a [`FormatSpec`] object to allow the usage of the additional format specifiers
    fn format_at<W: WriteBuffer>(
        &self,
        index: usize,
        w: &mut W,
        spec: &FormatSpec,
    ) -> AzUtilResult<()>;
}
impl FormatArgs for () {
    fn format_at<W: WriteBuffer>(
        &self,
        _index: usize,
        _w: &mut W,
        _spec: &FormatSpec,
    ) -> AzUtilResult<()> {
        Err(AzUtilErrorCode::FormatError)
    }
}
macro_rules! impl_format_args {
    ($($T:ident, $idx:tt),+) => {
        impl<$($T: FDisplay),+> FormatArgs for ($($T),+,) {
            #[allow(non_snake_case)]
            fn format_at<W: WriteBuffer>(&self, index: usize, w: &mut W, spec: &FormatSpec) -> AzUtilResult<()> {
                match index { $($idx => self.$idx.fmt(w, spec),)+ _ => Err(AzUtilErrorCode::ParseError) }
            }
        }
    };
}
impl_format_args!(T0, 0);
impl_format_args!(T0, 0, T1, 1);
impl_format_args!(T0, 0, T1, 1, T2, 2);
impl_format_args!(T0, 0, T1, 1, T2, 2, T3, 3);
impl_format_args!(T0, 0, T1, 1, T2, 2, T3, 3, T4, 4);
impl_format_args!(T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5);

/// String formatting function
///
/// Accepts a mutable buffer that implements the [`WriteBuffer`] trait, a format string, and any type of argument that implements the [`FormatArgs`] trait
/// Writes the formatted value to the buffer.
pub fn format_rt<W, A>(buf: &mut W, fmt: &str, args: &A) -> AzUtilResult<()>
where
    W: WriteBuffer,
    A: FormatArgs,
{
    let mut arg_idx = 0;
    let mut parts = fmt.split('{');
    if let Some(part) = parts.next() {
        buf.write_str(part)?;
    }

    for part in parts {
        if part.starts_with('{') {
            buf.write_str("{")?;
            buf.write_str(&part[1..])?;
            continue;
        }

        if let Some(end_brace_idx) = part.find('}') {
            let spec_str = &part[..end_brace_idx];
            let spec = FormatSpec::parse_spec(spec_str);

            args.format_at(arg_idx, buf, &spec)?;
            arg_idx += 1;

            buf.write_str(&part[end_brace_idx + 1..])?;
        } else {
            if !part.is_empty() {
                return Err(AzUtilErrorCode::FormatError);
            }
        }
    }

    Ok(())
}

/// Wrapper around the [`format_rt`] function to simplify the [`crate::format_str()`] macro definition
pub fn format_str<A: FormatArgs>(fmt: &str, args: &A) -> String {
    let mut buffer = AllocString::new();
    match format_rt(&mut buffer, fmt, args) {
        Ok(()) => buffer.into_string().expect(""),
        Err(e) => panic!("Failed to format value: {:?}", e),
    }
}
