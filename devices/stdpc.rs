// Already initialized by Novusk
use crate::fk_print;

#[no_mangle]
pub extern "C" fn fn_dev_name() -> &'static str {
    return "StdPC";
}

fn stdpc_init() { fk_print!("This device has already been initialized by Novusk\n"); }

fn stdpc_end() { fk_print!("Running on: {}\n", fn_dev_name()); }

module_init!(fn_dev_init, stdpc_init);
module_end!(fn_dev_end, stdpc_end);
