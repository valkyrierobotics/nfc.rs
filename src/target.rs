use crate::ffi;

pub mod target_info {
    pub use crate::ffi::{
        nfc_barcode_info as BarcodeInfo, nfc_dep_info as DepInfo, nfc_felica_info as FelicaInfo,
        nfc_iso14443a_info as Iso14443aInfo, nfc_iso14443b2ct_info as Iso14443b2ctInfo,
        nfc_iso14443b2sr_info as Iso14443b2srInfo, nfc_iso14443b_info as Iso14443bInfo,
        nfc_iso14443bi_info as Iso14443biInfo, nfc_jewel_info as JewelInfo,
    };
}

pub struct Target {
    pub baud_rate: crate::BaudRate,
    pub info: TargetInfo,
}

pub enum TargetInfo {
    ISO14443A { info: target_info::Iso14443aInfo },
    FELICA { info: target_info::FelicaInfo },
    ISO14443B { info: target_info::Iso14443bInfo },
    ISO14443BI { info: target_info::Iso14443biInfo },
    ISO14443B2SR { info: target_info::Iso14443b2srInfo },
    ISO14443B2CT { info: target_info::Iso14443b2ctInfo },
    JEWEL { info: target_info::JewelInfo },
    BARCODE { info: target_info::BarcodeInfo },
    DEP { info: target_info::DepInfo },
}

impl From<ffi::nfc_target> for Target {
    fn from(target: ffi::nfc_target) -> Target {
        let inner = unsafe {
            match target.modulation.modulation {
                ffi::nfc_modulation_type::ISO14443A => TargetInfo::ISO14443A {
                    info: target.target_info.nai,
                },
                ffi::nfc_modulation_type::ISO14443B => TargetInfo::ISO14443B {
                    info: target.target_info.nbi,
                },
                ffi::nfc_modulation_type::ISO14443BI => TargetInfo::ISO14443BI {
                    info: target.target_info.nii,
                },
                ffi::nfc_modulation_type::ISO14443B2SR => TargetInfo::ISO14443B2SR {
                    info: target.target_info.nsi,
                },
                ffi::nfc_modulation_type::ISO14443B2CT => TargetInfo::ISO14443B2CT {
                    info: target.target_info.nci,
                },
                ffi::nfc_modulation_type::FELICA => TargetInfo::FELICA {
                    info: target.target_info.nfi,
                },
                ffi::nfc_modulation_type::BARCODE => TargetInfo::BARCODE {
                    info: target.target_info.nti,
                },
                ffi::nfc_modulation_type::JEWEL => TargetInfo::JEWEL {
                    info: target.target_info.nji,
                },
                ffi::nfc_modulation_type::DEP => TargetInfo::DEP {
                    info: target.target_info.ndi,
                },
            }
        };

        Target {
            info: inner,
            baud_rate: target.modulation.baud_rate,
        }
    }
}

impl From<&ffi::nfc_target> for Target {
    fn from(target: &ffi::nfc_target) -> Target {
        Target::from(*target)
    }
}

impl Into<ffi::nfc_target> for Target {
    fn into(self) -> ffi::nfc_target {
        let specifics = match self.info {
            TargetInfo::ISO14443A { info } => (
                ffi::nfc_target_info { nai: info },
                ffi::nfc_modulation_type::ISO14443A,
            ),
            TargetInfo::FELICA { info } => (
                ffi::nfc_target_info { nfi: info },
                ffi::nfc_modulation_type::FELICA,
            ),
            TargetInfo::ISO14443B { info } => (
                ffi::nfc_target_info { nbi: info },
                ffi::nfc_modulation_type::ISO14443B,
            ),
            TargetInfo::ISO14443BI { info } => (
                ffi::nfc_target_info { nii: info },
                ffi::nfc_modulation_type::ISO14443BI,
            ),
            TargetInfo::ISO14443B2SR { info } => (
                ffi::nfc_target_info { nsi: info },
                ffi::nfc_modulation_type::ISO14443B2SR,
            ),
            TargetInfo::ISO14443B2CT { info } => (
                ffi::nfc_target_info { nci: info },
                ffi::nfc_modulation_type::ISO14443B2CT,
            ),
            TargetInfo::JEWEL { info } => (
                ffi::nfc_target_info { nji: info },
                ffi::nfc_modulation_type::JEWEL,
            ),
            TargetInfo::BARCODE { info } => (
                ffi::nfc_target_info { nti: info },
                ffi::nfc_modulation_type::BARCODE,
            ),
            TargetInfo::DEP { info } => (
                ffi::nfc_target_info { ndi: info },
                ffi::nfc_modulation_type::DEP,
            ),
        };

        ffi::nfc_target {
            target_info: specifics.0,
            modulation: ffi::nfc_modulation {
                modulation: specifics.1,
                baud_rate: self.baud_rate,
            },
        }
    }
}
