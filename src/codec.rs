use crate::errors::{AzUtilErrorCode, AzUtilResult};
use alloc::string::String;
use alloc::vec::Vec;

/// Trait for encoding and decoding data types to and from byte buffers.
pub trait Codec {
    /// Encodes `self` into the provided encoder.
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()>;
    /// Decodes an instance of `Self` from the provided decoder.
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self>
    where
        Self: Sized;
}

/// A generic encoder that serializes different primitive types and collections into a byte buffer.
#[derive(Clone)]
pub struct Encoder {
    buf: Vec<u8>,
}
impl Encoder {

    /// Creates a new, empty `Encoder`.
    #[inline(always)]
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Appends a single `u8` value to the buffer.
    #[inline(always)]
    pub fn push_u8(&mut self, v: u8) -> AzUtilResult<()> {
        self.buf.push(v);
        Ok(())
    }

    /// Encodes raw bytes by prefixing them with their length and appending them to the buffer.
    #[inline(always)]
    pub fn push_raw_bytes(&mut self, bytes: Vec<u8>) -> AzUtilResult<()> {
        self.push_u32(bytes.len() as u32)?;
        self.buf.extend(bytes);
        Ok(())
    }

    /// Encodes a `u16` value in big-endian format.
    #[inline(always)]
    pub fn push_u16(&mut self, v: u16) -> AzUtilResult<()> {
        self.buf.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    /// Encodes a `u32` value in big-endian format.
    #[inline(always)]
    pub fn push_u32(&mut self, v: u32) -> AzUtilResult<()> {
        self.buf.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    /// Encodes a `u64` value in big-endian format.
    #[inline(always)]
    pub fn push_u64(&mut self, v: u64) -> AzUtilResult<()> {
        self.buf.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    /// Encodes an `i64` value in big-endian format.
    #[inline(always)]
    pub fn push_i64(&mut self, v: i64) -> AzUtilResult<()> {
        self.buf.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    /// Encodes a `usize` value in big-endian format.
    #[inline(always)]
    pub fn push_usize(&mut self, v: usize) -> AzUtilResult<()> {
        let bytes = v.to_be_bytes();
        self.buf.extend_from_slice(&bytes[..size_of::<usize>()]);
        Ok(())    }

    /// Encodes an `Option<T>` by writing a presence flag (`1` or `0`) followed by the value if present.
    #[inline(always)]
    pub fn push_opt<T>(&mut self, t: &Option<T>) -> AzUtilResult<()>
    where
        T: Codec,
    {
        if let Some(v) = t {
            self.push_u8(1)?;
            v.encode(self)?;
        } else {
            self.push_u8(0)?;
        };
        Ok(())
    }

    /// Encodes a slice of `T` values by prefixing its length and encoding each element.
    #[inline(always)]
    pub fn push_slice<T>(&mut self, slice: &[T]) -> AzUtilResult<()>
    where
        T: Codec,
    {
        self.push_u32(slice.len() as u32)?;
        slice.iter().for_each(|v| v.encode(self).unwrap());
        Ok(())
    }

    /// Encodes a vector of `T` values, prefixing each element with its size before encoding it.
    #[inline(always)]
    pub fn push_vec<T>(&mut self, vec: Vec<T>) -> AzUtilResult<()>
    where
        T: Codec + Sized,
    {
        self.push_u32(vec.len() as u32)?;
        for i in vec.iter() {
            let t_size = size_of::<T>();
            self.push_u16(t_size as u16)?;
            i.encode(self)?;
        }
        Ok(())
    }

    /// Encodes a UTF-8 string, prefixing it with its length before writing its bytes.
    #[inline(always)]
    pub fn push_string(&mut self, s: &String) -> AzUtilResult<()> {
        let b = s.as_bytes();
        self.push_u32(b.len() as u32)?;
        self.buf.extend_from_slice(b);
        Ok(())
    }

    /// Encodes a boolean value as `1` (true) or `0` (false).
    #[inline(always)]
    pub fn push_bool(&mut self, b: bool) -> AzUtilResult<()> {
        self.push_u8(b as u8)
    }

    /// Encodes an `i8` value as a single byte.
    #[inline(always)]
    pub fn push_i8(&mut self, v: i8) -> AzUtilResult<()> {
        self.push_u8(v as u8)
    }

    /// Consumes the encoder and returns the encoded byte buffer.
    #[inline(always)]
    pub fn into_inner(self) -> Vec<u8> {
        self.buf
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

/// A generic decoder that deserializes primitive types and collections from a byte slice.
pub struct Decoder<'a> {
    buf: &'a [u8],
    cursor: usize,
}

impl<'a> Decoder<'a> {

    /// Creates a new `Decoder` for the given byte buffer.
    #[inline(always)]
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, cursor: 0 }
    }

    /// Reads a single `u8` from the buffer.
    #[inline(always)]
    pub fn read_u8(&mut self) -> AzUtilResult<u8> {
        if self.cursor >= self.buf.len() {
            return Err(AzUtilErrorCode::UnexpectedEOF);
        }
        let val = self.buf[self.cursor];
        self.cursor += 1;
        Ok(val)
    }

    /// Reads a `usize` in big-endian format.
    #[inline(always)]
    pub fn read_usize(&mut self) -> AzUtilResult<usize> {
        let n = size_of::<usize>();
        if self.cursor + n > self.buf.len() {
            return Err(AzUtilErrorCode::UnexpectedEOF);
        }
        let mut tmp = [0u8; size_of::<usize>()];
        tmp.copy_from_slice(&self.buf[self.cursor..self.cursor + n]);
        self.cursor += n;
        Ok(usize::from_be_bytes(tmp))
    }

    /// Reads a `u16` (2 bytes) in big-endian format.
    #[inline(always)]
    pub fn read_u16(&mut self) -> AzUtilResult<u16> {
        if self.cursor + 2 > self.buf.len() {
            return Err(AzUtilErrorCode::UnexpectedEOF);
        }
        let val = u16::from_be_bytes([self.buf[self.cursor], self.buf[self.cursor + 1]]);
        self.cursor += 2;
        Ok(val)
    }

    /// Reads a vector of elements of type `T` by reading its length and decoding each element.
    #[inline(always)]
    pub fn read_vec<T>(&mut self) -> AzUtilResult<Vec<T>>
    where
        T: Codec + Sized,
    {
        let len = self.read_u32()?;
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::decode(self)?);
        }
        Ok(vec)
    }

    /// Reads an `Option<T>` by checking the presence flag and decoding the value if present.
    #[inline(always)]
    pub fn read_opt<T: Codec>(&mut self) -> AzUtilResult<Option<T>> {
        let flag = self.read_u8()?;
        if flag == 0 {
            Ok(None)
        } else {
            Ok(Some(T::decode(self)?))
        }
    }

    /// Reads a slice of elements of type `T` as a vector.
    #[inline(always)]
    pub fn read_slice<T: Codec>(&mut self) -> AzUtilResult<Vec<T>> {
        let len = self.read_u32()? as usize;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(T::decode(self)?);
        }
        Ok(result)
    }

    /// Reads a `u32` (4 bytes) in big-endian format.
    #[inline(always)]
    pub fn read_u32(&mut self) -> AzUtilResult<u32> {
        if self.cursor + 4 > self.buf.len() {
            return Err(AzUtilErrorCode::UnexpectedEOF);
        }
        let val = u32::from_be_bytes([
            self.buf[self.cursor],
            self.buf[self.cursor + 1],
            self.buf[self.cursor + 2],
            self.buf[self.cursor + 3],
        ]);
        self.cursor += 4;
        Ok(val)
    }

    /// Reads an `i8` value.
    #[inline(always)]
    pub fn read_i8(&mut self) -> AzUtilResult<i8> {
        if self.cursor >= self.buf.len() {
            return Err(AzUtilErrorCode::UnexpectedEOF);
        }
        let val = self.buf[self.cursor] as i8;
        self.cursor += 1;
        Ok(val)
    }

    /// Reads an `i64` (8 bytes) in big-endian format.
    #[inline(always)]
    pub fn read_i64(&mut self) -> AzUtilResult<i64> {
        if self.cursor + 8 > self.buf.len() {
            return Err(AzUtilErrorCode::UnexpectedEOF);
        }
        let val = i64::from_be_bytes([
            self.buf[self.cursor],
            self.buf[self.cursor + 1],
            self.buf[self.cursor + 2],
            self.buf[self.cursor + 3],
            self.buf[self.cursor + 4],
            self.buf[self.cursor + 5],
            self.buf[self.cursor + 6],
            self.buf[self.cursor + 7],
        ]);
        self.cursor += 8;
        Ok(val)
    }
    
    /// Reads a `u64` (8 bytes) in big-endian format.
    #[inline(always)]
    pub fn read_u64(&mut self) -> AzUtilResult<u64> {
        if self.cursor + 8 > self.buf.len() {
            return Err(AzUtilErrorCode::UnexpectedEOF);
        }
        let val = u64::from_be_bytes([
            self.buf[self.cursor],
            self.buf[self.cursor + 1],
            self.buf[self.cursor + 2],
            self.buf[self.cursor + 3],
            self.buf[self.cursor + 4],
            self.buf[self.cursor + 5],
            self.buf[self.cursor + 6],
            self.buf[self.cursor + 7],
        ]);
        self.cursor += 8;
        Ok(val)
    }

    /// Reads a sequence of bytes of the specified length.
    #[inline(always)]
    pub fn read_bytes(&mut self, size: u32) -> AzUtilResult<Vec<u8>> {
        if self.cursor + size as usize > self.buf.len() {
            return Err(AzUtilErrorCode::UnexpectedEOF);
        }

        let bytes = self.buf[self.cursor..self.cursor + size as usize].to_vec();
        self.cursor += size as usize;
        Ok(bytes)
    }

    /// Reads a UTF-8 string prefixed with its length.
    #[inline(always)]
    pub fn read_string(&mut self) -> AzUtilResult<String> {
        let len = self.read_u32()? as usize;
        if self.cursor + len > self.buf.len() {
            return Err(AzUtilErrorCode::UnexpectedEOF);
        }
        let bytes = self.buf[self.cursor..self.cursor + len].to_vec();
        self.cursor += len;
        String::from_utf8(bytes).map_err(|_| AzUtilErrorCode::CodecError)
    }


    /// Reads a boolean value (`1` = true, `0` = false).
    #[inline(always)]
    pub fn read_bool(&mut self) -> AzUtilResult<bool> {
        let val = self.read_u8()?;
        Ok(val != 0)
    }
}
impl Codec for u8 {
    #[inline(always)]
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
        enc.push_u8(*self)
    }

    #[inline(always)]
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self>
    where
        Self: Sized,
    {
        dec.read_u8()
    }
}

impl Codec for u16 {
    #[inline(always)]
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
        enc.push_u16(*self)
    }
    #[inline(always)]
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self>
    where
        Self: Sized,
    {
        dec.read_u16()
    }
}

impl Codec for u32 {
    #[inline(always)]
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
        enc.push_u32(*self)
    }

    #[inline(always)]
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self>
    where
        Self: Sized,
    {
        dec.read_u32()
    }
}

impl Codec for u64 {
    #[inline(always)]
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
        enc.push_u64(*self)
    }
    #[inline(always)]
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self>
    where
        Self: Sized,
    {
        dec.read_u64()
    }
}

impl Codec for usize {
    #[inline(always)]
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
        enc.push_usize(*self)
    }
    #[inline(always)]
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self>
    where
        Self: Sized,
    {
        dec.read_usize()
    }
}

impl Codec for String {
    #[inline(always)]
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
        enc.push_string(self)
    }

    #[inline(always)]
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self>
    where
        Self: Sized,
    {
        dec.read_string()
    }
}

impl Codec for bool {
    #[inline(always)]
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
        enc.push_u8(if *self { 1 } else { 0 })
    }
    #[inline(always)]
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self>
    where
        Self: Sized,
    {
        Ok(dec.read_u8()? != 0)
    }
}

impl<T> Codec for Vec<T>
where
    T: Codec,
{
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
        enc.push_slice(self)
    }
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self>
    where
        Self: Sized,
    {
        dec.read_slice()
    }
}

impl<T> Codec for Option<T>
where
    T: Codec,
{
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
        enc.push_opt::<T>(self)?;
        Ok(())
    }
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self>
    where
        Self: Sized,
    {
        dec.read_opt::<T>()
    }
}

impl Codec for i8 {
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
        enc.push_i8(*self)
    }
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self>
    where
        Self: Sized,
    {
        dec.read_i8()
    }
}

impl Codec for i64 {
    fn encode(&self, enc: &mut Encoder) -> AzUtilResult<()> {
        enc.push_i64(*self)
    }
    fn decode(dec: &mut Decoder) -> AzUtilResult<Self> {
        dec.read_i64()
    }
}


