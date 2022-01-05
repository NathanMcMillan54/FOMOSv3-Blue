use crate::fk_print;
use crate::fomos::tasks::run_tasks;

fn main_tasks() {
    if unsafe { run_tasks() } == 1 {
        fk_print!("An error occurred in a main task");
    }
}

pub fn main_loop() {
    loop {
        main_tasks();
    }
}

