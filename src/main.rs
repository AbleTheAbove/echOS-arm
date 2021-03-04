#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

mod arch;
pub use arch::aarch64::qemu_virt::QEMU_UART0_VIRT;

mod kdriver;
// Kernel public functions
pub use kdriver::serial::{serial_log, serialc};

// Kernel private functions
use kdriver::uri_sys::init;

global_asm!(include_str!("start.s"));
#[no_mangle]
pub extern "C" fn boot() {
    serial_log("UART driver loaded");
    serial_log("uri-sys driver loaded");
    init();
    serial_log("kernel fully loaded");

    serial_log("SPIN CPU INIT");

    loop {}
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {}
}
