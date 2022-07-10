use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let device_file = if env::var_os("CARGO_FEATURE_EFM32PG12B500").is_some() {
            "src/efm32pg12b500/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32PG1B100").is_some() {
            "src/efm32pg1b100/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32PG1B200").is_some() {
            "src/efm32pg1b200/device.x"
        } else { panic!("No device features selected"); };

        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

