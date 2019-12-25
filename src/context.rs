use ::ffi;
use ::failure::{bail, Error};

use crate::device::NfcDevice;


pub struct NfcContext {
    raw_context: *mut ffi::nfc_context,
}

impl NfcContext {
    pub fn new() -> Result<Self, Error> {
        let mut new_context: *mut ffi::nfc_context = std::ptr::null_mut();
        unsafe {
            ffi::nfc_init(&mut new_context);
        }

        if new_context.is_null() {
            bail!("Creating a new context failed");
        }

        Ok(NfcContext{raw_context: new_context})
    }

    pub fn open_device(&mut self, connstring: &str) -> Result<NfcDevice, Error>{
        let mut connarr: [i8; 1024] = [0; 1024];
        let end = std::cmp::min(1024, connstring.len());
        connarr.copy_from_slice(&unsafe { &*(connstring.as_bytes() as *const _ as *const [i8]) }[0..end]);
        
        let device;
        unsafe {
            device = ffi::nfc_open(self.raw_context, &connarr);
        }

        if device.is_null() {
            bail!("Failed to allocate nfc_device")
        }

        Ok(NfcDevice::new(device, std::marker::PhantomData))
    }
}

impl Drop for NfcContext {
    fn drop(&mut self) {
        unsafe {
            ffi::nfc_exit(&mut self.raw_context);
        }
    }
}
