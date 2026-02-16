//! Build script for ESP32 examples

fn main() {
    println!("cargo:rerun-if-env-changed=ESP_IDF_VERSION");
    println!("cargo:rerun-if-env-changed=MCU");
    println!("cargo:rerun-if-changed=sdkconfig.defaults");

    // Propagate ESP-IDF linker args and cfg from esp-idf-sys to this binary crate
    embuild::espidf::sysenv::output();
}
