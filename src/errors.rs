use azathoth_core::errors::{AzError, ErrorClass};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AzUtilErrorCode {
    FormatError = 0x01,
    ParseError = 0x02,
    NotFound = 0x03,
    HashError = 0x04,
    CodecError = 0x05,
    UnexpectedEOF
}

impl core::fmt::Display for AzUtilErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FormatError => write!(f,"format error"),
            Self::NotFound => write!(f,"not found"),
            Self::ParseError => write!(f,"parse error"),
            Self::HashError => write!(f, "hash error"),
            Self::CodecError => write!(f, "codec error"),
            Self::UnexpectedEOF => write!(f, "unexpected EOF")
        }
    }
}

impl AzError for AzUtilErrorCode {
    const CLASS: ErrorClass = ErrorClass::Other;
    fn code(&self) -> u16 {
        *self as u16
    }
    fn is_retryable(&self) -> bool {
        false
    }
    fn os_code(&self) -> Option<u32> {
        None
    }
}

/// Result wrapper
pub type AzUtilResult<T> = Result<T, AzUtilErrorCode>;
