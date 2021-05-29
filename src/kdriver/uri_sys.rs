use crate::serial_log;

//const URI_BYTE_LENGTH: usize = 16;
/*
fn _resolve(_uri: &str) -> [u8; URI_BYTE_LENGTH] {
    return [0; URI_BYTE_LENGTH];
}
*/
pub fn init() {
    serial_log("> uri-sys driver loaded");
}
