use ::ffi;

use ::std::convert::From;

pub struct NfcDevice<'context> {
    raw_device: *mut ffi::nfc_device,
    _phantom: std::marker::PhantomData<&'context ffi::nfc_device>,
}

impl<'context> NfcDevice<'context> {
    pub ( crate ) fn new(raw_device: *mut ffi::nfc_device, _phantom: std::marker::PhantomData<&'context ffi::nfc_device>) -> Self {
        NfcDevice{raw_device, _phantom}
    }
}

pub struct NfcInitiator<'context> {
    raw_device: *mut ffi::nfc_device,
    _phantom: std::marker::PhantomData<&'context ffi::nfc_device>,
}

impl<'context> From<NfcDevice<'context>> for NfcInitiator<'context> {
    fn from(device: NfcDevice<'context>) -> Self {
        NfcInitiator{raw_device: device.raw_device, _phantom: device._phantom}
    }
}


impl<'context> Drop for NfcInitiator<'context> {
    fn drop(&mut self) {
        unsafe {
            ffi::nfc_close(self.raw_device);
        }
    }
}

impl<'context> Drop for NfcDevice<'context> {
    fn drop(&mut self) {
        unsafe {
            ffi::nfc_close(self.raw_device);
        }
    }
}
