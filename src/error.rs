use crate::ffi;
use std::fmt;

use std::error;

use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

const SUCCESS: i32 = 0;

pub type NfcResult<T> = Result<T, NfcError>;

trait Successful {
    fn is_success(&self) -> bool;
}

#[repr(i32)]
#[derive(Debug, Primitive)]
pub enum ErrorKind {
    Success = SUCCESS,
    InputOutput = ffi::NFC_EIO,
    InvalidArguments = ffi::NFC_EINVARG,
    OperationNotSupported = ffi::NFC_EDEVNOTSUPP,
    Overflow = ffi::NFC_EOVFLOW,
    Timeout = ffi::NFC_ETIMEOUT,
    OperationAborted = ffi::NFC_EOPABORTED,
    NotImplemented = ffi::NFC_ENOTIMPL,
    TargetReleased = ffi::NFC_ETGRELEASED,
    RFTransmission = ffi::NFC_ERFTRANS,
    MifareClassicAuth = ffi::NFC_EMFCAUTHFAIL,
    Software = ffi::NFC_ESOFT,
    InternalChip = ffi::NFC_ECHIP,
}

impl ErrorKind {
    fn from_foreign(data: i32) -> Option<Self> {
        ErrorKind::from_i32(data)
    }
}

// Would prefer to use nfc_strerror but it doesn't take an error code just
// gets the most recent.
//
// I may pull request that change into libnfc later.
impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            ErrorKind::Success => "Success",
            ErrorKind::InputOutput => "Input / Output Error",
            ErrorKind::InvalidArguments => "Invalid argument(s)",
            ErrorKind::OperationNotSupported => "Not Supported by Device",
            ErrorKind::Overflow => "Buffer Overflow",
            ErrorKind::Timeout => "Timeout",
            ErrorKind::OperationAborted => "Operation Aborted",
            ErrorKind::NotImplemented => "Not (yet) Implemented",
            ErrorKind::TargetReleased => "Target Released",
            ErrorKind::MifareClassicAuth => "Mifare Authentication Error",
            ErrorKind::RFTransmission => "RF Transmission Error",
            ErrorKind::InternalChip => "Device's Internal Chip Error",
            _ => "Unknown error",
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub enum NfcError {
    FfiError { error: FfiError },
    UnknownError { details: String },
}

#[derive(Debug)]
pub struct FfiError {
    error: ErrorKind,
}

impl error::Error for NfcError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl NfcError {
    // only let people construct the unknown version
    pub fn new(message: &str) -> Self {
        NfcError::UnknownError {
            details: message.to_string(),
        }
    }
}

impl From<i32> for NfcError {
    fn from(data: i32) -> Self {
        match ErrorKind::from_foreign(data) {
            Some(error) => NfcError::FfiError {
                error: FfiError { error },
            },
            None => NfcError::UnknownError {
                details: "FFI returned gibberish error code".to_string(),
            },
        }
    }
}

impl fmt::Display for NfcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NfcError::FfiError { error } => write!(f, "NFC Error Occurred: {}", error.error),
            NfcError::UnknownError { details } => {
                write!(f, "Unknown NFC error occurred: {}", details)
            }
        }
    }
}
