// Application state and orchestration.

use std::io::{self, Write};
use crate::ui::{green, cyan, yellow, red, bold, dim};
use crate::storage;
use crate::task::{Task, Priority, Status};
use crate::journal::JournalEntry;
use crate::note::Note;
use crate::review::DailyReview;

pub struct App {
    tasks: Vec<Task>,
    journal_entries: Vec<JournalEntry>,
    notes: Vec<Note>,
    reviews: Vec<DailyReview>,
}

impl App {
    pub fn new() -> Self {
        let tasks = storage::load_tasks();
        let journal_entries = storage::load_journal_entries();
        let notes = storage::load_notes();
        let reviews = storage::load_reviews();

        Self {
            tasks,
            journal_entries,
            notes,
            reviews,
        }
    }

    pub fn journal_prompt(&mut self) -> io::Result<()> {
        println!();
        println!("    {} {}", green("📖"), bold("Journal"));

        let entries = &self.journal_entries;
        if entries.is_empty() {
            println!("    {}", dim("  No entries yet."));
        } else {
            println!("    {}", dim("  Recent entries:"));
            for entry in entries.iter().take(5) {
                println!("    {} {} - {}", dim("  •"), entry.date(), entry.content());
            }
            if entries.len() > 5 {
                println!("    {} {}", dim("  •"), dim(&format!("... and {} more", entries.len() - 5)));
            }
        }

        println!();
        print!("    {} ", cyan("✏️"));
        io::stdout().flush()?;

        let mut content = String::new();
        io::stdin().read_line(&mut content)?;
        let content = content.trim();

        if !content.is_empty() {
            let date = chrono::Local::now().format("%Y-%m-%d").to_string();
            let entry = JournalEntry::new(&date, content);
            match storage::save_journal_entry(&entry) {
                Ok(_) => {
                    self.journal_entries.push(entry);
                    println!("    {} Journal entry saved!", green("✓"));
                }
                Err(e) => println!("    {} Error saving: {}", red("✗"), e),
            }
        }
        println!();
        Ok(())
    }

    pub fn task_prompt(&mut self) -> io::Result<()> {
        println!();
        println!("    {} {}", green("✅"), bold("Tasks"));

        let tasks = &self.tasks;
        let active: Vec<_> = tasks.iter().filter(|t| t.is_active()).collect();
        let completed: Vec<_> = tasks.iter().filter(|t| t.is_completed()).collect();

        if active.is_empty() && completed.is_empty() {
            println!("    {}", dim("  No tasks yet."));
        } else {
            if !active.is_empty() {
                println!("    {}", dim("  Active:"));
                for (i, task) in active.iter().enumerate() {
                    let status_icon = match task.status() {
                        Status::Pending => "○",
                        Status::InProgress => "◉",
                        Status::Blocked => "⊘",
                        _ => "?",
                    };
                    let priority_color = match task.priority() {
                        Priority::Low => dim,
                        Priority::Medium => |s: &str| s.to_string(),
                        Priority::High => yellow,
                        Priority::Critical => red,
                    };
                    let title = priority_color(task.title());
                    println!("    {} {} {}", dim(&format!("{}:", i + 1)), dim(status_icon), title);
                    if let Some(origin) = task.origin() {
                        println!("    {} {}", dim("  →"), dim(origin));
                    }
                }
            }

            if !completed.is_empty() {
                println!();
                println!("    {}", dim("  Completed:"));
                for task in completed.iter().take(5) {
                    println!("    {} {}", dim("✓"), dim(task.title()));
                }
                if completed.len() > 5 {
                    println!("    {} {}", dim("  •"), dim(&format!("... and {} more", completed.len() - 5)));
                }
            }
        }

        println!();
        println!("    {}", dim("  Commands:"));
        println!("    {} {}", dim("  •"), "add <title>");
        println!("    {} {}", dim("  •"), "done <number>");
        println!("    {} {}", dim("  •"), "block <number>");
        println!("    {} {}", dim("  •"), "cancel <number>");
        println!("    {} {}", dim("  •"), "reopen <number>");
        println!();

        print!("    {} ", cyan("⚡"));
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            return Ok(());
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];

        match command {
            "add" => {
                if let Some(title) = parts.get(1) {
                    let priority = Priority::Medium;
                    let mut task = Task::new(title, priority);

                    // Ask for origin
                    print!("    {} ", cyan("📝 Origin (optional): "));
                    io::stdout().flush()?;
                    let mut origin = String::new();
                    io::stdin().read_line(&mut origin)?;
                    let origin = origin.trim();
                    if !origin.is_empty() {
                        task.set_origin(origin);
                    }

                    match storage::save_task(&task) {
                        Ok(_) => {
                            self.tasks.push(task);
                            println!("    {} Task created!", green("✓"));
                        }
                        Err(e) => println!("    {} Error saving: {}", red("✗"), e),
                    }
                } else {
                    println!("    {} Usage: add <title>", yellow("⚠"));
                }
            }
            "done" => {
                if let Some(idx_str) = parts.get(1) {
                    if let Ok(idx) = idx_str.parse::<usize>() {
                        let mut active: Vec<_> = self.tasks
                            .iter_mut()
                            .filter(|t| t.is_active())
                            .collect();
                        if let Some(task) = active.get_mut(idx - 1) {
                            match task.complete() {
                                Ok(_) => {
                                    // Ask for completion reflection
                                    print!("    {} ", cyan("💭 Why did this feel good? "));
                                    io::stdout().flush()?;
                                    let mut completion = String::new();
                                    io::stdin().read_line(&mut completion)?;
                                    let completion = completion.trim();
                                    if !completion.is_empty() {
                                        task.set_completion(completion);
                                    }

                                    let _ = storage::save_task(task);
                                    println!("    {} Task completed!", green("✓"));
                                }
                                Err(e) => println!("    {} Error: {}", red("✗"), e),
                            }
                        } else {
                            println!("    {} Task not found", yellow("⚠"));
                        }
                    } else {
                        println!("    {} Invalid number", yellow("⚠"));
                    }
                } else {
                    println!("    {} Usage: done <number>", yellow("⚠"));
                }
            }
            "block" => {
                if let Some(idx_str) = parts.get(1) {
                    if let Ok(idx) = idx_str.parse::<usize>() {
                        let mut active: Vec<_> = self.tasks
                            .iter_mut()
                            .filter(|t| t.is_active())
                            .collect();
                        if let Some(task) = active.get_mut(idx - 1) {
                            match task.block() {
                                Ok(_) => {
                                    let _ = storage::save_task(task);
                                    println!("    {} Task blocked!", green("✓"));
                                }
                                Err(e) => println!("    {} Error: {}", red("✗"), e),
                            }
                        } else {
                            println!("    {} Task not found", yellow("⚠"));
                        }
                    } else {
                        println!("    {} Invalid number", yellow("⚠"));
                    }
                } else {
                    println!("    {} Usage: block <number>", yellow("⚠"));
                }
            }
            "cancel" => {
                if let Some(idx_str) = parts.get(1) {
                    if let Ok(idx) = idx_str.parse::<usize>() {
                        let mut active: Vec<_> = self.tasks
                            .iter_mut()
                            .filter(|t| t.is_active())
                            .collect();
                        if let Some(task) = active.get_mut(idx - 1) {
                            match task.cancel() {
                                Ok(_) => {
                                    let _ = storage::save_task(task);
                                    println!("    {} Task cancelled!", green("✓"));
                                }
                                Err(e) => println!("    {} Error: {}", red("✗"), e),
                            }
                        } else {
                            println!("    {} Task not found", yellow("⚠"));
                        }
                    } else {
                        println!("    {} Invalid number", yellow("⚠"));
                    }
                } else {
                    println!("    {} Usage: cancel <number>", yellow("⚠"));
                }
            }
            "reopen" => {
                if let Some(idx_str) = parts.get(1) {
                    if let Ok(idx) = idx_str.parse::<usize>() {
                        let mut completed: Vec<_> = self.tasks
                            .iter_mut()
                            .filter(|t| t.is_completed())
                            .collect();
                        if let Some(task) = completed.get_mut(idx - 1) {
                            match task.reopen() {
                                Ok(_) => {
                                    let _ = storage::save_task(task);
                                    println!("    {} Task reopened!", green("✓"));
                                }
                                Err(e) => println!("    {} Error: {}", red("✗"), e),
                            }
                        } else {
                            println!("    {} Task not found", yellow("⚠"));
                        }
                    } else {
                        println!("    {} Invalid number", yellow("⚠"));
                    }
                } else {
                    println!("    {} Usage: reopen <number>", yellow("⚠"));
                }
            }
            _ => {
                println!("    {} Unknown command: {}", yellow("⚠"), command);
            }
        }

        println!();
        Ok(())
    }

    pub fn note_prompt(&mut self) -> io::Result<()> {
        println!();
        println!("    {} {}", green("📓"), bold("Notes"));

        let notes = &self.notes;
        if notes.is_empty() {
            println!("    {}", dim("  No notes yet."));
        } else {
            println!("    {}", dim("  Recent fragments:"));
            for note in notes.iter().take(10) {
                println!("    {} {}", dim("  •"), note.fragment());
                if let Some(context) = note.context() {
                    println!("    {} {}", dim("    →"), dim(context));
                }
            }
            if notes.len() > 10 {
                println!("    {} {}", dim("  •"), dim(&format!("... and {} more", notes.len() - 10)));
            }
        }

        println!();
        println!("    {}", dim("  Commands:"));
        println!("    {} {}", dim("  •"), "add <fragment>");
        println!();

        print!("    {} ", cyan("✏️"));
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            return Ok(());
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];

        match command {
            "add" => {
                if let Some(fragment) = parts.get(1) {
                    let mut note = Note::new(fragment);

                    // Ask for context
                    print!("    {} ", cyan("📍 Context (optional): "));
                    io::stdout().flush()?;
                    let mut context = String::new();
                    io::stdin().read_line(&mut context)?;
                    let context = context.trim();
                    if !context.is_empty() {
                        note = note.with_context(context);
                    }

                    match storage::save_note(&note) {
                        Ok(_) => {
                            self.notes.push(note);
                            println!("    {} Note saved!", green("✓"));
                        }
                        Err(e) => println!("    {} Error saving: {}", red("✗"), e),
                    }
                } else {
                    println!("    {} Usage: add <fragment>", yellow("⚠"));
                }
            }
            _ => {
                // If no command, treat as a new note
                let note = Note::new(input);
                match storage::save_note(&note) {
                    Ok(_) => {
                        self.notes.push(note);
                        println!("    {} Note saved!", green("✓"));
                    }
                    Err(e) => println!("    {} Error saving: {}", red("✗"), e),
                }
            }
        }

        println!();
        Ok(())
    }

    pub fn review_prompt(&mut self) -> io::Result<()> {
        println!();
        println!("    {} {}", green("🔄"), bold("Daily Review"));

        let date = chrono::Local::now().format("%Y-%m-%d").to_string();

        // Show today's journal entries
        let today_entries: Vec<_> = self.journal_entries
            .iter()
            .filter(|e| e.date() == date)
            .collect();

        if !today_entries.is_empty() {
            println!("    {}", dim("  Today you wrote:"));
            for entry in today_entries {
                println!("    {} {}", dim("  •"), entry.content());
                if let Some(mood) = entry.mood() {
                    println!("    {} {}", dim("    →"), dim(&format!("Mood: {}", mood)));
                }
                if let Some(question) = entry.question() {
                    println!("    {} {}", dim("    →"), dim(&format!("Question: {}", question)));
                }
            }
        } else {
            println!("    {}", dim("  No journal entries today yet."));
        }

        // Show tasks completed today (we approximate by looking at completed tasks)
        let completed_today: Vec<_> = self.tasks
            .iter()
            .filter(|t| t.is_completed())
            .collect();

        if !completed_today.is_empty() {
            println!();
            println!("    {}", dim("  You completed:"));
            for task in completed_today {
                println!("    {} {}", dim("  ✓"), task.title());
                if let Some(completion) = task.completion() {
                    println!("    {} {}", dim("    →"), dim(completion));
                }
            }
        }

        // Show pending tasks
        let pending: Vec<_> = self.tasks
            .iter()
            .filter(|t| t.is_active())
            .collect();

        if !pending.is_empty() {
            println!();
            println!("    {}", dim("  Tomorrow begins with:"));
            for task in pending.iter().take(5) {
                let priority_color = match task.priority() {
                    Priority::Low => dim,
                    Priority::Medium => |s: &str| s.to_string(),
                    Priority::High => yellow,
                    Priority::Critical => red,
                };
                println!("    {} {}", dim("  •"), priority_color(task.title()));
            }
            if pending.len() > 5 {
                println!("    {} {}", dim("  •"), dim(&format!("... and {} more", pending.len() - 5)));
            }
        }

        // Show recent notes
        let recent_notes: Vec<_> = self.notes
            .iter()
            .rev()
            .take(3)
            .collect();

        if !recent_notes.is_empty() {
            println!();
            println!("    {}", dim("  Recent fragments:"));
            for note in recent_notes {
                println!("    {} {}", dim("  •"), note.fragment());
            }
        }

        // Reflection
        println!();
        println!("    {}", dim("  Reflection:"));
        print!("    {} ", cyan("💭"));
        io::stdout().flush()?;

        let mut reflection = String::new();
        io::stdin().read_line(&mut reflection)?;
        let reflection = reflection.trim();

        if !reflection.is_empty() {
            let review = DailyReview::new(&date).with_reflection(reflection);
            match storage::save_review(&review) {
                Ok(_) => {
                    self.reviews.push(review);
                    println!("    {} Review saved!", green("✓"));
                }
                Err(e) => println!("    {} Error saving: {}", red("✗"), e),
            }
        }

        println!();
        Ok(())
    }
}
