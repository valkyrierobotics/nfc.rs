//! # NFC.rs
//!
//! NFC.rs wraps the popular libnfc C library in idiomatic Rust, making it both safer
//! and easier to use.
//!

////////////////////////////////////////////////////////////////////////////////

mod context;
mod device;
mod error;
mod ffi;
mod target;
mod util;

pub use ffi::{
    nfc_baud_rate as BaudRate, nfc_dep_info as DepInfo, nfc_dep_mode as DepMode,
    nfc_modulation as Modulation, nfc_modulation_type as ModulationType, nfc_property as Property,
};

pub use context::Context;
pub use device::{Device, Initiator, PollType, TargetAndCount, TargetResultEnum};
pub use target::{Target, TargetInfo};

pub use target::target_info;

pub use error::{NfcError as Error, NfcResult as Result};

/// Retrieves the version of the linked NFC library.
pub fn version() -> &'static str {
    unsafe {
        std::ffi::CStr::from_ptr(ffi::nfc_version())
            .to_str()
            .unwrap()
    }
}
