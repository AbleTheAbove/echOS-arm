#![no_std]
#![no_main]
#![feature(global_asm)]

mod arch;
mod panic;

// The virt arch
global_asm!(include_str!("arch/aarch64/qemu_virt/start.s"));
pub use arch::aarch64::qemu_virt::QEMU_UART0_VIRT;

mod kdriver;

// Kernel public functions
pub use kdriver::serial::{serial_log, serialc};
use kdriver::{kfs, uri_sys};

mod config;

pub const CONFIG: &str = include_str!("/home/able/Projects/echOS-arm/root/kernel_config.json");
/// The main function called at runtime by the assembly
#[no_mangle]
pub extern "C" fn boot() {
    uri_sys::init();
    kfs::init();
    config::get_config();
    serial_log("> kernel fully loaded");
    serial_log("SPIN CPU INFINITLY");
    loop {} // If commented out the kernel panics
}
