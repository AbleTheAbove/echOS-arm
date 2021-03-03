#![no_std]
#![no_main]
#![feature(global_asm)]

use core::{panic::PanicInfo, ptr};

const QEMU_UART0_VIRT: *mut u8 = 0x0900_0000 as *mut u8;

global_asm!(include_str!("start.s"));
#[no_mangle]
pub extern "C" fn boot() {
    serial_log("Loading the kernel\n");
    serial_log("Kernel Loaded\n");
    loop {}
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn serial_log(str: &str) {
    let base: u64 = 0xFE000000 + 0x7e20100;

    for byte in str.bytes() {
        unsafe {
            ptr::write_volatile(base as *mut u8, byte);
        }
    }
}
