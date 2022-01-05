use libfktask::{Task, TaskManager};

pub static mut TASKMANAGER: TaskManager = TaskManager {
    tasks: vec![],
    total_tasks: 0,
    running_task: 0
};

pub unsafe fn run_tasks() -> i32 {
    let result = TASKMANAGER.run_all_tasks();

    if result.is_err() {
        return 1;
    } else { return 0; }
}
