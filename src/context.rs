use crate::ffi;

use crate::device::Device;

use crate::{Error, Result};

pub struct Context {
    raw_context: *mut ffi::nfc_context,
}

impl Context {
    pub fn new() -> Self {
        let mut new_context: *mut ffi::nfc_context = std::ptr::null_mut();
        unsafe {
            ffi::nfc_init(&mut new_context);
        }

        if new_context.is_null() {
            // for context: the standard library assumes malloc
            // will never fail, and that's the only way context
            // can be null, so at that point either we'll crash
            // or something else will
            panic!("Context should never be null");
        }

        Context {
            raw_context: new_context,
        }
    }

    pub fn open_device(&mut self, connstring: &str) -> Result<Device> {
        let mut connarr = crate::util::str_to_connarr(connstring);

        let device;
        unsafe {
            device = ffi::nfc_open(self.raw_context, connarr.as_mut_ptr());
        }

        if device.is_null() {
            // for context, unfortunately we don't get any error info
            // from trying to open a device, we just get a perror, which
            // is pleasant.
            Err(Error::new(
                "Unable to open device, check STDERR for details!",
            ))
        } else {
            Ok(Device {
                raw_device: device,
                _phantom: std::marker::PhantomData,
            })
        }
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
