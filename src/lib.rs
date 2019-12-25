pub use rnfc_sys as ffi;

mod context;
mod device;
mod target;
mod util;

pub use ffi::{
    nfc_baud_rate as BaudRate, nfc_modulation as Modulation, nfc_modulation_type as ModulationType,
};

pub use context::Context;
pub use device::{Device, Initiator, PollType};
pub use failure::Error;
pub use target::Target;

pub fn version() -> &'static str {
    unsafe {
        std::ffi::CStr::from_ptr(ffi::nfc_version()).to_str().unwrap()
    }
}
