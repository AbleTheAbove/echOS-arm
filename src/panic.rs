use crate::serial_log;
use core::panic::PanicInfo;
#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    serial_log("Kernel crashed, looping forever");
    loop {} // try handling the reset of the os
}
