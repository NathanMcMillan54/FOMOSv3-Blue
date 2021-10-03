#[no_mangle]
pub extern "C" fn kernel_config() -> &'static str {
    return include_str!("config.txt")
}

#[no_mangle]
pub extern "C" fn initramfs_main() {

}
