#![no_std]
#![no_main]
#![feature(global_asm)]

use core::{panic::PanicInfo, ptr};

mod arch;
use arch::aarch64::qemu_virt::QEMU_UART0_VIRT;

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

fn serial_log(str: &str) {
    for byte in str.bytes() {
        serialc(byte)
    }
    serialc(0x0A); // Adds a \n to the end of serial_log
}

fn serialc(byte: u8) {
    let uart_register = QEMU_UART0_VIRT;
    unsafe {
        ptr::write_volatile(uart_register as *mut u8, byte);
    }
}
