use crate::ffi;
use failure::{bail, Error};

use crate::device::Device;

pub struct Context {
    raw_context: *mut ffi::nfc_context,
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        let mut new_context: *mut ffi::nfc_context = std::ptr::null_mut();
        unsafe {
            ffi::nfc_init(&mut new_context);
        }

        if new_context.is_null() {
            bail!("Creating a new context failed");
        }

        Ok(Context {
            raw_context: new_context,
        })
    }

    pub fn open_device(&mut self, connstring: &str) -> Result<Device, Error> {
        let connarr = crate::util::str_to_connarr(connstring);

        let device;
        unsafe {
            device = ffi::nfc_open(self.raw_context, &connarr);
        }

        if device.is_null() {
            bail!("Failed to allocate nfc_device")
        }

        Ok(Device {
            raw_device: device,
            _phantom: std::marker::PhantomData,
        })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if !self.raw_context.is_null() {
            unsafe {
                ffi::nfc_exit(self.raw_context);
            }
        }
    }
}
