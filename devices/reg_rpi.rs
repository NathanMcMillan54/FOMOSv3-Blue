// Already initialized by Novusk
use crate::fk_print;

#[no_mangle]
pub extern "C" fn fn_dev_name() -> &'static str {
    return "Reg RPi";
}

fn rpi_init() {  }

fn rpi_end() {    }

module_init!(fn_dev_init, rpi_init);
module_end!(fn_dev_end, rpi_end);
