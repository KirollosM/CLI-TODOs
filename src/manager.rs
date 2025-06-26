use crate::task::{Status, Task};

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: vec![] }
    }

    pub fn add_task(&mut self, title: String) {
        let new_id: usize = if let Some(last) = self.tasks.last() {
            last.id + 1
        } else {
            1
        };

        let task: Task = Task::new(new_id, title.clone());
        self.tasks.push(task);
        println!("Task {} added!", title);
    }

    pub fn remove_task(&mut self, id: usize) -> Result<(), String> {
        if let Some(pos) = self.tasks.iter().position(|task: &Task| task.id == id) {
            self.tasks.remove(pos);
            println!("Task with id: {}, removed!", id);
            Ok(())
        } else {
            Err(format!("Task with id {} not found.", id))
        }
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            for task in &self.tasks {
                let status_str = match task.status {
                    Status::Pending => "Pending",
                    Status::Completed => "Completed",
                };

                println!("[{}] {} - {}", task.id, task.title, status_str);
            }
        }
    }

    pub fn complete_task(&mut self, id: usize) -> Result<(), String> {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                if let Status::Completed = task.status {
                    println!("Task with id {} is already completed", task.id);
                } else {
                    task.complete();
                    println!("Task with id {} has been set to completed", task.id);
                }
                Ok(())
            }
            None => Err(format!("Task with id {} not found", id)),
        }
    }
}
