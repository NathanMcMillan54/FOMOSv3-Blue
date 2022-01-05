use libfktask::Task;
use crate::fomos::tasks::TASKMANAGER;

pub fn add_arch_tasks() {
    unsafe {
        TASKMANAGER.add_task(Task {
            name: "Kb Mouse Input",
            id: 0,
            task: super::input::kbmouse::mouse_input,
        })
    }
}