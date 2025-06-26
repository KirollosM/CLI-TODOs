#[derive(Debug)]
pub enum Status {
    Pending,
    Completed,
}

#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub status: Status,
}

impl Task {
    pub fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            status: Status::Pending,
        }
    }

    pub fn complete(&mut self) {
        self.status = Status::Completed;
    }

    // pub fn iscomplete(&self) -> &Status {
    //     println!(
    //         "The current status of {} is {:?}",
    //         &self.title, &self.status
    //     );
    //     &self.status
    // }
}
