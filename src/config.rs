use crate::{serial_log, CONFIG};
use serde::Deserialize;
use serde_json_core;
#[derive(Deserialize, Debug)]
pub struct KernelConfig {
    pub banner: bool,
}
pub fn get_config<'a>() {
    match serde_json_core::from_str::<'a, KernelConfig>(CONFIG) {
        Ok(_) => serial_log("hi"),
        Err(e) => serial_log("Kernel config error."),
    }
}
