// Storage - a shelf that knows how to keep things.
// It stores records. It doesn't know what they mean.

use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;

const DATA_DIR: &str = "data";

pub fn ensure_data_dir() -> io::Result<()> {
    if !Path::new(DATA_DIR).exists() {
        fs::create_dir(DATA_DIR)?;
    }
    Ok(())
}

// A record is a list of strings, separated by "---"
pub fn append_record(kind: &str, lines: &[String]) -> io::Result<()> {
    ensure_data_dir()?;
    let path = format!("{}/{}.txt", DATA_DIR, kind);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    writeln!(file, "---")?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

pub fn load_records(kind: &str) -> Vec<Vec<String>> {
    let path = format!("{}/{}.txt", DATA_DIR, kind);
    let path = Path::new(&path);
    if !path.exists() {
        return Vec::new();
    }

    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);

    let mut records = Vec::new();
    let mut current = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        if line == "---" {
            if !current.is_empty() {
                records.push(current);
                current = Vec::new();
            }
        } else {
            current.push(line);
        }
    }
    if !current.is_empty() {
        records.push(current);
    }

    records
}

// Convenience functions that use the record format
pub fn save_task(task: &crate::task::Task) -> io::Result<()> {
    append_record("tasks", &task.to_record())
}

pub fn load_tasks() -> Vec<crate::task::Task> {
    let records = load_records("tasks");
    records
        .iter()
        .filter_map(|r| crate::task::Task::from_record(r).ok())
        .collect()
}

pub fn save_journal_entry(entry: &crate::journal::JournalEntry) -> io::Result<()> {
    append_record("journal", &entry.to_record())
}

pub fn load_journal_entries() -> Vec<crate::journal::JournalEntry> {
    let records = load_records("journal");
    records
        .iter()
        .filter_map(|r| crate::journal::JournalEntry::from_record(r).ok())
        .collect()
}

pub fn save_note(note: &crate::note::Note) -> io::Result<()> {
    append_record("notes", &note.to_record())
}

pub fn load_notes() -> Vec<crate::note::Note> {
    let records = load_records("notes");
    records
        .iter()
        .filter_map(|r| crate::note::Note::from_record(r).ok())
        .collect()
}

pub fn save_review(review: &crate::review::DailyReview) -> io::Result<()> {
    append_record("reviews", &review.to_record())
}

pub fn load_reviews() -> Vec<crate::review::DailyReview> {
    let records = load_records("reviews");
    records
        .iter()
        .filter_map(|r| crate::review::DailyReview::from_record(r).ok())
        .collect()
}
