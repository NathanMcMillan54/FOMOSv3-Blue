extern "C" {
    fn fn_dev_name() -> &'static str;
}

// For now
fn sdev() { }
fn edev() { }

module_init!(fn_device_init, sdev);
module_end!(fn_device_end, edev);

pub unsafe fn device_init() {
    start_module!(fn_dev_init, fn_dev_end);


}
