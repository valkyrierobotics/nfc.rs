#![allow(non_snake_case)]

use crate::ffi;
use bit_vec::BitVec;

use crate::{BaudRate, DepInfo, DepMode, Modulation, Property, Target};

use failure::{bail, Error};
use std::convert::TryInto;

pub struct Device<'context> {
    pub(crate) raw_device: *mut ffi::nfc_device,
    pub(crate) _phantom: std::marker::PhantomData<&'context ffi::nfc_device>,
}

impl<'context> Device<'context> {
    pub fn set_bool_property(&mut self, property: Property, enable: bool) -> Result<(), Error> {
        let res;
        unsafe {
            res = ffi::nfc_device_set_property_bool(self.raw_device, property, enable);
        }

        if res != 0 {
            bail!("Failed to set bool property")
        }
        Ok(())
    }

    pub fn set_int_property(&mut self, property: Property, value: ffi::c_int) -> Result<(), Error> {
        let res;
        unsafe {
            res = ffi::nfc_device_set_property_int(self.raw_device, property, value);
        }

        if res != 0 {
            bail!("Failed to set int property")
        }
        Ok(())
    }
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
        let mut target: ffi::nfc_target = unsafe { std::mem::zeroed() };
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
                0: TargetAndCount { count, target: target.into() },
            })
        }
    }

    // TODO add a better modulations enum that takes care of the pbtInitData?
    pub fn select_passive_target(
        &mut self,
        modulation: Modulation,
        pbtInitData: &[u8],
    ) -> TargetResult {
        let mut target: ffi::nfc_target = unsafe { std::mem::zeroed() };
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
                0: TargetAndCount { count, target: target.into() },
            })
        }
    }

    pub fn list_passive_targets(
        &mut self,
        modulation: Modulation,
        max_targets: ffi::size_t,
    ) -> Vec<Target> {
        let mut targets: Vec<ffi::nfc_target> = Vec::with_capacity(max_targets);

        // first resize the vector up to the requested maximum
        targets.resize(max_targets, unsafe { std::mem::zeroed() });
        let count;
        unsafe {
            // Safety: raw_device can only be non-null,
            //  targets must be the right length as we check for the len and resize it.
            count = ffi::nfc_initiator_list_passive_targets(
                self.raw_device,
                modulation,
                targets.as_mut_ptr(),
                targets.len(),
            )
        }

        targets.resize(count.try_into().unwrap(), unsafe { std::mem::zeroed() });
        targets.iter().map(|iter| iter.into()).collect::<Vec<Target>>()
    }

    pub fn select_dep_target(
        &mut self,
        ndm: DepMode,
        nbr: BaudRate,
        dep_info: DepInfo,
        timeout: ffi::c_int,
    ) -> TargetResult {
        let mut target: ffi::nfc_target = unsafe { std::mem::zeroed() };
        let count;
        unsafe {
            count = ffi::nfc_initiator_select_dep_target(
                self.raw_device,
                ndm,
                nbr,
                &dep_info,
                &mut target,
                timeout,
            );
        }

        if count == 0 {
            Ok(TargetResultEnum::Empty)
        } else if count < 0 {
            bail!("Error while trying to select DEP target")
        } else {
            Ok(TargetResultEnum::Found {
                0: TargetAndCount { count, target: target.into() },
            })
        }
    }

    pub fn target_is_present(&mut self, target: Target) -> bool {
        unsafe { ffi::nfc_initiator_target_is_present(self.raw_device, &target.into()) == 0 }
    }

    pub fn last_target_is_present(&mut self) -> bool {
        unsafe { ffi::nfc_initiator_target_is_present(self.raw_device, std::ptr::null()) == 0 }
    }

    pub fn set_bool_property(&mut self, property: Property, enable: bool) -> Result<(), Error> {
        let res;
        unsafe {
            res = ffi::nfc_device_set_property_bool(self.raw_device, property, enable);
        }

        if res != 0 {
            bail!("Failed to set bool property")
        }
        Ok(())
    }

    pub fn set_int_property(&mut self, property: Property, value: ffi::c_int) -> Result<(), Error> {
        let res;
        unsafe {
            res = ffi::nfc_device_set_property_int(self.raw_device, property, value);
        }

        if res != 0 {
            bail!("Failed to set int property")
        }
        Ok(())
    }

    pub fn transceive_bytes(
        &mut self,
        send: &[u8],
        receive_size: ffi::size_t,
        timeout: ffi::c_int,
    ) -> Result<Vec<u8>, Error> {
        let mut received: Vec<u8> = vec![0; receive_size];
        let res;
        unsafe {
            res = ffi::nfc_initiator_transceive_bytes(
                self.raw_device,
                send.as_ptr(),
                send.len(),
                received.as_mut_ptr(),
                received.len(),
                timeout,
            )
        }

        // TOOD better error handling for EOVFLOW
        if res < 0 {
            bail!("Error during byte transaction")
        } else {
            Ok(received)
        }
    }

    pub fn transceive_bits(
        &mut self,
        send: &BitVec,
        parity_bits: &BitVec,
        receive_bits: ffi::size_t,
    ) -> Result<(BitVec, BitVec), Error> {
        let mut received: Vec<u8> = vec![0; receive_bits];
        let mut parity: Vec<u8> = vec![0; receive_bits / 8];
        let res;
        unsafe {
            res = ffi::nfc_initiator_transceive_bits(
                self.raw_device,
                send.to_bytes().as_ptr(),
                send.len(),
                parity_bits
                    .iter()
                    .map(|iter| iter as u8)
                    .collect::<Vec<u8>>()
                    .as_ptr(),
                received.as_mut_ptr(),
                received.len(),
                parity.as_mut_ptr(),
            )
        }

        // TOOD better error handling for EOVFLOW
        if res < 0 {
            bail!("Error during bit transaction")
        } else {
            Ok((
                BitVec::from_bytes(&mut received),
                BitVec::from_bytes(&mut parity),
            ))
        }
    }

    pub fn deselect_target(&mut self) -> Result<(), Error>{
        let ret = unsafe { ffi::nfc_initiator_deselect_target(self.raw_device) };
        if ret >= 0 { Ok(()) }
        else { bail!("Error when deselecting target") }
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
