pub use rnfc_sys as ffi;

mod context;
mod device;
mod target;
mod util;

pub use ffi::{
    nfc_baud_rate as BaudRate, nfc_dep_info as DepInfo, nfc_dep_mode as DepMode,
    nfc_modulation as Modulation, nfc_modulation_type as ModulationType, nfc_property as Property,
};

pub use context::Context;
pub use device::{Device, Initiator, PollType, TargetResultEnum, TargetAndCount};
pub use failure::Error;
pub use target::{Target, TargetInfo};

pub use target::target_info;

pub fn version() -> &'static str {
    unsafe {
        std::ffi::CStr::from_ptr(ffi::nfc_version())
            .to_str()
            .unwrap()
    }
}
