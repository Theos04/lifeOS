// A Task is a promise that remembers whether it has been kept.

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Pending,
    InProgress,
    Blocked,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct Task {
    title: String,
    status: Status,
    priority: Priority,
    origin: Option<String>,     // The sentence that caused this task to exist.
    completion: Option<String>, // Why completing this felt good.
}

impl Task {
    pub fn new(title: &str, priority: Priority) -> Self {
        Self {
            title: title.to_string(),
            status: Status::Pending,
            priority,
            origin: None,
            completion: None,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn priority(&self) -> &Priority {
        &self.priority
    }

    pub fn origin(&self) -> Option<&str> {
        self.origin.as_deref()
    }

    pub fn completion(&self) -> Option<&str> {
        self.completion.as_deref()
    }

    pub fn set_origin(&mut self, origin: &str) {
        self.origin = Some(origin.to_string());
    }

    pub fn set_completion(&mut self, completion: &str) {
        self.completion = Some(completion.to_string());
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn complete(&mut self) -> Result<(), &'static str> {
        match self.status {
            Status::Completed => Err("Task is already completed"),
            Status::Cancelled => Err("Cancelled tasks cannot be completed"),
            _ => {
                self.status = Status::Completed;
                Ok(())
            }
        }
    }

    pub fn block(&mut self) -> Result<(), &'static str> {
        match self.status {
            Status::Blocked => Err("Task is already blocked"),
            Status::Completed => Err("Completed tasks cannot be blocked"),
            Status::Cancelled => Err("Cancelled tasks cannot be blocked"),
            _ => {
                self.status = Status::Blocked;
                Ok(())
            }
        }
    }

    pub fn cancel(&mut self) -> Result<(), &'static str> {
        match self.status {
            Status::Cancelled => Err("Task is already cancelled"),
            Status::Completed => Err("Completed tasks cannot be cancelled"),
            _ => {
                self.status = Status::Cancelled;
                Ok(())
            }
        }
    }

    pub fn reopen(&mut self) -> Result<(), &'static str> {
        match self.status {
            Status::Pending => Err("Task is already pending"),
            Status::InProgress => Err("Task is already in progress"),
            Status::Completed => {
                self.status = Status::Pending;
                Ok(())
            }
            Status::Cancelled => {
                self.status = Status::Pending;
                Ok(())
            }
            Status::Blocked => {
                self.status = Status::Pending;
                Ok(())
            }
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        match self.status {
            Status::InProgress => Err("Task is already in progress"),
            Status::Completed => Err("Completed tasks cannot be started"),
            Status::Cancelled => Err("Cancelled tasks cannot be started"),
            _ => {
                self.status = Status::InProgress;
                Ok(())
            }
        }
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, Status::Completed)
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            Status::Pending | Status::InProgress | Status::Blocked
        )
    }

    // --- Representation ---
    // The domain knows how to become a record and how to parse from one.

    pub fn to_record(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("Title: {}", self.title));
        lines.push(format!("Status: {:?}", self.status));
        lines.push(format!("Priority: {:?}", self.priority));
        if let Some(origin) = &self.origin {
            lines.push(format!("Origin: {}", origin));
        }
        if let Some(completion) = &self.completion {
            lines.push(format!("Completion: {}", completion));
        }
        lines
    }

    pub fn from_record(lines: &[String]) -> Result<Self, &'static str> {
        let mut title = None;
        let mut status = None;
        let mut priority = None;
        let mut origin = None;
        let mut completion = None;

        for line in lines {
            if let Some(stripped) = line.strip_prefix("Title: ") {
                title = Some(stripped.to_string());
            } else if let Some(stripped) = line.strip_prefix("Status: ") {
                status = Some(stripped.to_string());
            } else if let Some(stripped) = line.strip_prefix("Priority: ") {
                priority = Some(stripped.to_string());
            } else if let Some(stripped) = line.strip_prefix("Origin: ") {
                origin = Some(stripped.to_string());
            } else if let Some(stripped) = line.strip_prefix("Completion: ") {
                completion = Some(stripped.to_string());
            }
        }

        let title = title.ok_or("Missing title")?;
        let status_str = status.ok_or("Missing status")?;
        let priority_str = priority.ok_or("Missing priority")?;

        let status = match status_str.as_str() {
            "Pending" => Status::Pending,
            "InProgress" => Status::InProgress,
            "Blocked" => Status::Blocked,
            "Completed" => Status::Completed,
            "Cancelled" => Status::Cancelled,
            _ => return Err("Invalid status"),
        };

        let priority = match priority_str.as_str() {
            "Low" => Priority::Low,
            "Medium" => Priority::Medium,
            "High" => Priority::High,
            "Critical" => Priority::Critical,
            _ => return Err("Invalid priority"),
        };

        let mut task = Task::new(&title, priority);
        task.status = status;
        if let Some(o) = origin {
            task.origin = Some(o);
        }
        if let Some(c) = completion {
            task.completion = Some(c);
        }
        Ok(task)
    }
}
