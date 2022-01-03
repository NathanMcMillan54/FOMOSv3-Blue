pub struct Task {
    pub name: &'static str,
    pub id: u32,
    pub task: extern "C" fn() -> Result<(), &'static str>,
}

impl Task {
    fn new(task_name: &'static str, task_id: u32, task_fun: extern "C" fn() -> Result<(), &'static str>) -> Self {
        return Task {
            name: task_name,
            id: task_id,
            task: task_fun,
        };
    }

    pub fn start_task(&self) -> Result<(), &'static str> {
        let ret = (self.task)();

        return ret;
    }
}
