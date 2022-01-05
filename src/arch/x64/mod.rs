use crate::fk_print;

pub(self) mod input;
pub(self) mod tasks;

use tasks::add_arch_tasks;

pub fn arch_setup() {
    fk_print!("Setting up x86_64 specific processes...\n");

    add_arch_tasks();
}
