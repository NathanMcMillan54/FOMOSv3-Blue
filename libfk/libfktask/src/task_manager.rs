use alloc::vec::Vec;
use core::ops::Add;
use crate::task::Task;

pub struct TaskManager {
    pub tasks: Vec<Task>,
    pub total_tasks: u32,
    pub running_task: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        return TaskManager {
            tasks: vec![],
            total_tasks: 0,
            running_task: 0,
        };
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);

        self.total_tasks += 1;
    }

    pub fn run_task(&mut self, id: u32) -> Result<(), &'static str> {
        let mut ret: Result<(), &'static str> = Ok(());

        for t in 0..self.total_tasks as usize {
            if self.tasks[t].id == id {
                self.running_task = id;
                ret = (self.tasks[t].task)();
                self.total_tasks -= 1;
            }
        }

        return ret;
    }

    pub fn run_all_tasks(&mut self) -> Result<(), &'static str> {
        let mut ret = Ok(());

        for t in 0..self.total_tasks as usize {
            let result = self.run_task(self.tasks[t].id);

            if result.is_err() {
                ret = Err(result.err().unwrap());
            }
        }

        return Ok(());
    }
}
