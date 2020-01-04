#![allow(non_snake_case)]

use crate::ffi;
use bit_vec::BitVec;

use crate::{BaudRate, DepInfo, DepMode, Error, Modulation, Property, Result, Target};

use std::convert::TryInto;

pub struct Device<'context> {
    pub(crate) raw_device: *mut ffi::nfc_device,
    pub(crate) _phantom: std::marker::PhantomData<&'context ffi::nfc_device>,
}

impl<'context> Device<'context> {
    pub fn set_bool_property(&mut self, property: Property, enable: bool) -> Result<()> {
        match unsafe { ffi::nfc_device_set_property_bool(self.raw_device, property, enable) } {
            0 => Ok(()),
            res => Err(Error::from(res)),
        }
    }

    pub fn set_int_property(&mut self, property: Property, value: ffi::c_int) -> Result<()> {
        match unsafe { ffi::nfc_device_set_property_int(self.raw_device, property, value) } {
            0 => Ok(()),
            res => Err(Error::from(res)),
        }
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

impl<'context> ::std::ops::Deref for Initiator<'context> {
    type Target = Device<'context>;

    fn deref(&self) -> &Device<'context> {
        &self.device
    }
}

impl<'context> ::std::ops::DerefMut for Initiator<'context> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.device
    }
}

// TODO probably make this not into (something that can fail)
impl<'context> Into<Result<SecureInitiator<'context>>> for Device<'context> {
    fn into(mut self) -> Result<SecureInitiator<'context>> {
        let result = unsafe { ffi::nfc_initiator_init_secure_element(self.raw_device) };

        if result != 0 {
            Err(Error::from(result))
        } else {
            let dev_copy = self.raw_device;
            self.raw_device = std::ptr::null_mut();
            Ok(SecureInitiator {
                0: Initiator {
                    device: Device {
                        raw_device: dev_copy,
                        _phantom: std::marker::PhantomData,
                    },
                },
            })
        }
    }
}

pub struct Initiator<'context> {
    device: Device<'context>,
}

impl<'context> Into<Result<Initiator<'context>>> for Device<'context> {
    fn into(mut self) -> Result<Initiator<'context>> {
        let result = unsafe { ffi::nfc_initiator_init(self.raw_device) };

        if result != 0 {
            Err(Error::from(result))
        } else {
            let dev_copy = self.raw_device;
            self.raw_device = std::ptr::null_mut();
            Ok(Initiator {
                device: Device {
                    raw_device: dev_copy,
                    _phantom: std::marker::PhantomData,
                },
            })
        }
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

// TODO not this
type TargetResult = Result<TargetResultEnum>;

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
                self.device.raw_device,
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
            Err(Error::from(count))
        } else {
            Ok(TargetResultEnum::Found {
                0: TargetAndCount {
                    count,
                    target: target.into(),
                },
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
        let count = unsafe {
            ffi::nfc_initiator_select_passive_target(
                self.device.raw_device,
                modulation,
                pbtInitData.as_ptr(),
                pbtInitData.len(),
                &mut target,
            )
        };

        if count == 0 {
            Ok(TargetResultEnum::Empty)
        } else if count < 0 {
            Err(Error::from(count))
        } else {
            Ok(TargetResultEnum::Found {
                0: TargetAndCount {
                    count,
                    target: target.into(),
                },
            })
        }
    }

    pub fn list_passive_targets(
        &mut self,
        modulation: Modulation,
        max_targets: ffi::size_t,
    ) -> Result<Vec<Target>> {
        let mut targets: Vec<ffi::nfc_target> = Vec::with_capacity(max_targets);

        // first resize the vector up to the requested maximum
        targets.resize(max_targets, unsafe { std::mem::zeroed() });
        let count = unsafe {
            // Safety: raw_device can only be non-null,
            //  targets must be the right length as we check for the len and resize it.
            ffi::nfc_initiator_list_passive_targets(
                self.device.raw_device,
                modulation,
                targets.as_mut_ptr(),
                targets.len(),
            )
        };

        if count < 0 {
            Err(Error::from(count))
        } else {
            targets.resize(count.try_into().unwrap(), unsafe { std::mem::zeroed() });
            Ok(targets
                .iter()
                .map(|iter| iter.into())
                .collect::<Vec<Target>>())
        }
    }

    pub fn select_dep_target(
        &mut self,
        ndm: DepMode,
        nbr: BaudRate,
        dep_info: DepInfo,
        timeout: ffi::c_int,
    ) -> TargetResult {
        let mut target: ffi::nfc_target = unsafe { std::mem::zeroed() };
        let count = unsafe {
            ffi::nfc_initiator_select_dep_target(
                self.device.raw_device,
                ndm,
                nbr,
                &dep_info,
                &mut target,
                timeout,
            )
        };

        if count == 0 {
            Ok(TargetResultEnum::Empty)
        } else if count < 0 {
            Err(Error::from(count))
        } else {
            Ok(TargetResultEnum::Found {
                0: TargetAndCount {
                    count,
                    target: target.into(),
                },
            })
        }
    }

    pub fn target_is_present(&mut self, target: Target) -> bool {
        unsafe { ffi::nfc_initiator_target_is_present(self.device.raw_device, &target.into()) == 0 }
    }

    pub fn last_target_is_present(&mut self) -> bool {
        unsafe {
            ffi::nfc_initiator_target_is_present(self.device.raw_device, std::ptr::null()) == 0
        }
    }

    pub fn transceive_bytes(
        &mut self,
        send: &[u8],
        receive_size: ffi::size_t,
        timeout: ffi::c_int,
    ) -> Result<Vec<u8>> {
        let mut received: Vec<u8> = vec![0; receive_size];
        let res = unsafe {
            ffi::nfc_initiator_transceive_bytes(
                self.device.raw_device,
                send.as_ptr(),
                send.len(),
                received.as_mut_ptr(),
                received.len(),
                timeout,
            )
        };

        // TOOD better error handling for EOVFLOW (i.e we should be able to
        // catch it and recover?
        if res < 0 {
            Err(Error::from(res))
        } else {
            Ok(received)
        }
    }

    pub fn transceive_bits(
        &mut self,
        send: &BitVec,
        parity_bits: &BitVec,
        receive_bits: ffi::size_t,
    ) -> Result<(BitVec, BitVec)> {
        let mut received: Vec<u8> = vec![0; receive_bits];
        let mut parity: Vec<u8> = vec![0; receive_bits / 8];
        let res = unsafe {
            ffi::nfc_initiator_transceive_bits(
                self.device.raw_device,
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
        };

        // TOOD better error handling for EOVFLOW
        if res < 0 {
            Err(Error::from(res))
        } else {
            Ok((
                BitVec::from_bytes(&mut received),
                BitVec::from_bytes(&mut parity),
            ))
        }
    }

    pub fn deselect_target(&mut self) -> Result<()> {
        let ret = unsafe { ffi::nfc_initiator_deselect_target(self.device.raw_device) };
        if ret >= 0 {
            Ok(())
        } else {
            Err(Error::from(ret))
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
