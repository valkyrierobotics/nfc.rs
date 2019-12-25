extern crate libc;
extern crate rnfc_sys as ffi;

mod context;
mod device;

pub use context::{NfcContext};
pub use device::{NfcDevice, NfcInitiator};
