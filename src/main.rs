#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

mod arch;
pub use arch::aarch64::qemu_virt::QEMU_UART0_VIRT;
mod kdriver;
use kdriver::serial::serial_log;

global_asm!(include_str!("start.s"));
#[no_mangle]
pub extern "C" fn boot() -> ! {
    serial_log("UART driver loaded");

    loop {}
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {}
}
