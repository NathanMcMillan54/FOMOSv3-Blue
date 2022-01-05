pub mod device;

pub unsafe fn setup_fomos() {
    device::device_init();
}
