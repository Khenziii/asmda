mod tasks;

use std::thread;
use std::mem::take;
use tasks::{Task, get_tasks};

pub struct Scheduler {
    tasks: Vec<Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        let tasks = get_tasks();

        Self { tasks }
    }

    pub fn run(&mut self) {
        let tasks = take(&mut self.tasks);
        for mut task in tasks {
            thread::spawn(move || loop {
                let time_until_next_run = task.get_time_until_next_run();
                thread::sleep(time_until_next_run);

                task.run();
            });
        }
    }
}
