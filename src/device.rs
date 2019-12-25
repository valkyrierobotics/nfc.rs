#![allow(non_snake_case)]

use crate::ffi;

use crate::{Modulation, Target};

use failure::{bail, Error};
use std::convert::TryInto;

pub struct Device<'context> {
    pub(crate) raw_device: *mut ffi::nfc_device,
    pub(crate) _phantom: std::marker::PhantomData<&'context ffi::nfc_device>,
}

pub struct SecureInitiator<'context>(Initiator<'context>);

impl<'context> ::std::ops::Deref for SecureInitiator<'context> {
    type Target = Initiator<'context>;

    fn deref(&self) -> &Initiator<'context> {
        &self.0
    }
}

impl<'context> ::std::ops::DerefMut for SecureInitiator<'context> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'context> Into<Result<SecureInitiator<'context>, Error>> for Device<'context> {
    fn into(mut self) -> Result<SecureInitiator<'context>, Error> {
        let result;
        unsafe { result = ffi::nfc_initiator_init_secure_element(self.raw_device) }

        if result != 0 {
            bail!("Failed to initiate as an initiator");
        }

        let ret = Ok(SecureInitiator {
            0: Initiator {
                raw_device: self.raw_device,
                _phantom: self._phantom,
            },
        });
        self.raw_device = std::ptr::null_mut();
        ret
    }
}

pub struct Initiator<'context> {
    raw_device: *mut ffi::nfc_device,
    _phantom: std::marker::PhantomData<&'context ffi::nfc_device>,
}

impl<'context> Into<Result<Initiator<'context>, Error>> for Device<'context> {
    fn into(mut self) -> Result<Initiator<'context>, Error> {
        let result;
        unsafe { result = ffi::nfc_initiator_init(self.raw_device) }

        if result != 0 {
            bail!("Failed to initiate as an initiator");
        }

        let res = Ok(Initiator {
            raw_device: self.raw_device,
            _phantom: self._phantom,
        });
        self.raw_device = std::ptr::null_mut();
        res
    }
}

pub enum PollType {
    Limited(u8),
    Forever,
}

pub struct TargetAndCount {
    pub count: ffi::c_int,
    pub target: Target,
}

pub enum TargetResultEnum {
    Empty,
    Found(TargetAndCount),
}

impl Into<Target> for TargetAndCount {
    fn into(self) -> Target {
        self.target
    }
}

type TargetResult = Result<TargetResultEnum, Error>;

impl<'context> Initiator<'context> {
    pub fn poll_target(
        &mut self,
        modulations: &[Modulation],
        poll_number: PollType,
        poll_period: u8,
    ) -> TargetResult {
        let count;
        // Safety: this is safe because if we don't get a target we return empty
        let mut target: Target = unsafe { std::mem::zeroed() };
        let pollnumber = match poll_number {
            PollType::Limited(len) => len,
            PollType::Forever => 0xFF,
        };

        unsafe {
            // Safety: as the device and context are forced to remain in scope,
            //  we are always able to safely make this call.
            count = ffi::nfc_initiator_poll_target(
                self.raw_device,
                modulations.as_ptr(),
                modulations.len(),
                pollnumber,
                poll_period,
                &mut target,
            );
        }

        if count == 0 {
            Ok(TargetResultEnum::Empty)
        } else if count < 0 {
            bail!("Error during polling");
        } else {
            Ok(TargetResultEnum::Found {
                0: TargetAndCount { count, target },
            })
        }
    }

    // TODO add a better modulations enum that takes care of the pbtInitData?
    pub fn select_passive_target(
        &mut self,
        modulation: Modulation,
        pbtInitData: &[u8],
    ) -> TargetResult {
        let mut target: Target = unsafe { std::mem::zeroed() };
        let count;

        unsafe {
            count = ffi::nfc_initiator_select_passive_target(
                self.raw_device,
                modulation,
                pbtInitData.as_ptr(),
                pbtInitData.len(),
                &mut target,
            );
        }

        if count == 0 {
            Ok(TargetResultEnum::Empty)
        } else if count < 0 {
            bail!("Error while trying to select passive target")
        } else {
            Ok(TargetResultEnum::Found {
                0: TargetAndCount { count, target },
            })
        }
    }

    pub fn list_passive_targets(&mut self, modulation: Modulation, max_targets: ffi::size_t) -> Vec<Target> {
        let mut targets: Vec<Target> = Vec::with_capacity(max_targets);
        targets.resize(max_targets, unsafe { std::mem::zeroed() });
        let count;
        unsafe {
            count = ffi::nfc_initiator_list_passive_targets(self.raw_device,
                                                                modulation,
                                                                targets.as_mut_ptr(),
                                                                targets.len())
        }

        targets.resize(count.try_into().unwrap(), unsafe { std::mem::zeroed() });
        targets
    }
}

impl<'context> Drop for Initiator<'context> {
    fn drop(&mut self) {
        if !self.raw_device.is_null() {
            unsafe {
                ffi::nfc_close(self.raw_device);
            }
        }
    }
}

impl<'context> Drop for Device<'context> {
    fn drop(&mut self) {
        if !self.raw_device.is_null() {
            unsafe {
                ffi::nfc_close(self.raw_device);
            }
        }
    }
}
