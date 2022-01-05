#[cfg(feature = "stdpc")]
#[path = "../../../devices/stdpc.rs"]
pub(crate) mod stdpc;

#[cfg(feature = "rpi")]
#[path = "../../../devices/reg_rpi.rs"]
pub(crate) mod rpi;

extern "C" {
    fn fn_dev_name() -> &'static str;
}

pub unsafe fn device_init() {
    start_module!(fn_dev_init, fn_dev_end);
}
