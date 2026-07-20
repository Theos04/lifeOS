mod ui; 

use std::io::{self, Write};
use std::fs::{self, OpenOptions};
use std::path::Path;

use ui::{green, cyan, yellow, red, bold, dim, clear_screen};


// File helpers
fn read_entries(file_path: &str) -> Vec<String> {
    if !Path::new(file_path).exists() {
        return Vec::new();
    }
    
    match fs::read_to_string(file_path) {
        Ok(content) => content.lines().map(String::from).collect(),
        Err(_) => Vec::new(),
    }
}

fn append_entry(file_path: &str, entry: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    writeln!(file, "{}", entry)?;
    Ok(())
}

fn main() -> io::Result<()> {
    // Clear screen
    print!("\x1b[2J\x1b[H");
    io::stdout().flush()?;

    // Header
    let border = "═".repeat(42);
    
    println!("╔{}╗", border);
    println!("║{:^42}║", "");
    
    let title = bold("✦ LIFE ✦");
    println!("║{:^42}║", cyan(&title));
    
    let subtitle = dim("your daily companion");
    println!("║{:^42}║", subtitle);
    println!("║{:^42}║", "");
    println!("╚{}╝", border);
    println!();

    // Menu
    println!("    {} {}", green("●"), "Journal");
    println!("    {} {}", green("●"), "Notes");
    println!("    {} {}", red("●"), "Quit");
    println!();
    println!("    {}", dim("─────────────────────────────"));
    println!();

    // Prompt loop
    loop {
        print!("    {} ", cyan("➜"));
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "j" | "journal" => {
                println!();
                println!("    {} {}", green("📖"), bold("Journal"));
                
                let journal_file = "journal.txt";
                let entries = read_entries(journal_file);
                
                if entries.is_empty() {
                    println!("    {}", dim("  No entries yet."));
                } else {
                    println!("    {}", dim("  Recent entries:"));
                    for entry in entries.iter().take(5) {
                        println!("    {} {}", dim("  •"), entry);
                    }
                    if entries.len() > 5 {
                        println!("    {} {}", dim("  •"), dim(&format!("... and {} more", entries.len() - 5)));
                    }
                }
                
                println!();
                print!("    {} ", cyan("✏️"));
                io::stdout().flush()?;
                
                let mut entry = String::new();
                io::stdin().read_line(&mut entry)?;
                let entry = entry.trim();
                
                if !entry.is_empty() {
                    match append_entry(journal_file, entry) {
                        Ok(_) => println!("    {} Entry saved!", green("✓")),
                        Err(e) => println!("    {} Error saving: {}", red("✗"), e),
                    }
                }
                println!();
            }
            "n" | "notes" => {
                println!();
                println!("    {} {}", green("📓"), bold("Notes"));
                
                let notes_file = "notes.txt";
                let entries = read_entries(notes_file);
                
                if entries.is_empty() {
                    println!("    {}", dim("  No notes yet."));
                } else {
                    println!("    {}", dim("  Recent notes:"));
                    for entry in entries.iter().take(5) {
                        println!("    {} {}", dim("  •"), entry);
                    }
                    if entries.len() > 5 {
                        println!("    {} {}", dim("  •"), dim(&format!("... and {} more", entries.len() - 5)));
                    }
                }
                
                println!();
                print!("    {} ", cyan("✏️"));
                io::stdout().flush()?;
                
                let mut entry = String::new();
                io::stdin().read_line(&mut entry)?;
                let entry = entry.trim();
                
                if !entry.is_empty() {
                    match append_entry(notes_file, entry) {
                        Ok(_) => println!("    {} Note saved!", green("✓")),
                        Err(e) => println!("    {} Error saving: {}", red("✗"), e),
                    }
                }
                println!();
            }
            "q" | "quit" => {
                println!();
                println!("    {} {}", green("✨"), bold("Goodbye!"));
                println!();
                break;
            }
            "" => {}
            _ => {
                println!("    {} Unknown command: {}", yellow("⚠"), input);
                println!("    {} Try: journal, notes, or quit", dim("→"));
                println!();
            }
        }
    }

    Ok(())
}
