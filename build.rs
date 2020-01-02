use bindgen;

use std::env;
use std::path::PathBuf;

const VARS: &[&str] = &[
    "NFC_SUCCESS",
    "NFC_EIO",
    "NFC_EINVARG",
    "NFC_EDEVNOTSUPP",
    "NFC_ENOTSUCHDEV",
    "NFC_EOVFLOW",
    "NFC_ETIMEOUT",
    "NFC_EOPABORTED",
    "NFC_ENOTIMPL",
    "NFC_ETGRELEASED",
    "NFC_ERFTRANS",
    "NFC_EMFCAUTHFAIL",
    "NFC_ESOFT",
    "NFC_ECHIP",
];

const TYPES: &[&str] = &[
    "nfc_modulation_type",
    "nfc_modulation",
    "nfc_baud_rate",
    "nfc_property",
    "nfc_mode",
    "nfc_device",
    "nfc_user_defined_device",
    "nfc_context",
    "nfc_target",
    "nfc_driver",
];

const FUNCTIONS: &[&str] = &[
    "nfc_init",
    "nfc_exit",
    "nfc_open",
    "nfc_close",
    "nfc_abort_command",
    "nfc_list_devices",
    "nfc_idle",
    "nfc_version",
    "nfc_initiator_init",
    "nfc_initiator_init_secure_element",
    "nfc_initiator_select_passive_target",
    "nfc_initiator_list_passive_targets",
    "nfc_initiator_poll_target",
    "nfc_initiator_select_dep_target",
    "nfc_initiator_poll_dep_target",
    "nfc_register_driver",
    "nfc_abort_command",
    "nfc_list_devices",
    "iso14443b_crc_append",
    "iso14443b_crc",
    "nfc_devie_get_information_about",
    "iso14443a_crc_append",
    "iso14443a_crc",
    "nfc_device_get_supported_baud_rate_target_mode",
    "nfc_device_get_supported_baud_rate",
    "nfc_device_get_supported_modulation",
    "nfc_device_get_connstring",
    "nfc_device_get_name",
    "nfc_device_get_last_error",
    "nfc_device_set_property_bool",
    "nfc_device_set_property_int",
    "nfc_target_receive_bits",
    "nfc_target_send_bits",
    "nfc_target_receive_bytes",
    "nfc_target_send_bytes",
    "nfc_target_init",
    "nfc_initiator_transceive_bits_timed",
    "nfc_initiator_transceive_bytes_timed",
    "nfc_initiator_transceive_bytes",
    "nfc_initiator_transceive_bits",
    "nfc_initiator_target_is_present",
    "nfc_initiator_deselect_target",
    "nfc_strerror",
    "nfc_device_get_last_error",
    // TODO consider perror/strerror
];

fn main() {
    println!("cargo:rustc-link-lib=nfc");

    println!("cargo:rerun-if-changed=src/bindgen.h");

    let mut bindings = bindgen::Builder::default()
        .header("src/bindgen.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .ctypes_prefix("::libc");

    for whitelisted_type in TYPES {
        bindings = bindings.whitelist_type(whitelisted_type)
    }
    for whitelisted_fn in FUNCTIONS {
        bindings = bindings.whitelist_function(whitelisted_fn)
    }
    for whitelisted_var in VARS {
        bindings = bindings.whitelist_var(whitelisted_var)
    }

    let final_bindings = bindings
        // explicitly define a list of types to whitelist because otherwise
        // we get a bunch of libc bindings
        .rustified_enum(".*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    final_bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
