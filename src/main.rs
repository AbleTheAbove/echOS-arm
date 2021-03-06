#![no_std]
#![no_main]
#![feature(global_asm)]

mod arch;
mod panic;

// The virt arch
global_asm!(include_str!("arch/aarch64/qemu_virt/start.s"));
pub use arch::aarch64::qemu_virt::QEMU_UART0_VIRT;
//

mod kdriver;
// Kernel public functions
pub use kdriver::serial::{serial_log, serialc};
use kdriver::{kfs::get_metadata, uri_sys};
// Kernel private functions

use kdriver::kencrypt;

#[no_mangle]
pub extern "C" fn boot() {
    serial_log("UART driver loaded");
    uri_sys::init();
    serial_log("uri-sys driver loaded");
    let _kfsmd = get_metadata();
    serial_log("kernel file system driver loaded");

    // test kencrypt
    kencrypt::test_encrypt();
    serial_log("kencrypt driver loaded");

    serial_log("kernel fully loaded");
    /*
        serial_log("SPIN CPU INFINITLY");
        loop {}  // If commented out the kernel panics
    */
}
