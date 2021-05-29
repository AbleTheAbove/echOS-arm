use core::ptr;

use crate::QEMU_UART0_VIRT;

/// Logs an &str to the UART
pub fn serial_log(str: &str) {
    for byte in str.bytes() {
        serialc(byte)
    }
    serialc(0x0A); // Adds a \n to the end of serial_log
}
/// Logs a single character to the UART
pub fn serialc(byte: u8) {
    let uart_register = QEMU_UART0_VIRT;
    unsafe {
        ptr::write_volatile(uart_register as *mut u8, byte);
    }
}
