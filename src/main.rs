// LifeOS - A place for fragments of commitment, experience, reflection, and thought.
// Architecture: Domain → Representation → Persistence → Representation → Domain

mod ui;
mod storage;
mod task;
mod journal;
mod note;
mod review;
mod app;

use std::io::{self, Write};
use ui::{green, cyan, yellow, red, bold, dim};
use app::App;

fn main() -> io::Result<()> {
    let mut app = App::new();

    // Clear screen
    print!("\x1b[2J\x1b[H");
    io::stdout().flush()?;

    // Header
    let border = "═".repeat(42);

    println!("╔{}╗", border);
    println!("║{:^42}║", "");

    let title = bold("✦ LIFEOS ✦");
    println!("║{:^42}║", cyan(&title));

    let subtitle = dim("where fragments live");
    println!("║{:^42}║", subtitle);
    println!("║{:^42}║", "");
    println!("╚{}╝", border);
    println!();

    // Menu
    println!("    {} {}", green("●"), "Journal");
    println!("    {} {}", green("●"), "Tasks");
    println!("    {} {}", green("●"), "Notes");
    println!("    {} {}", green("●"), "Review");
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
            "j" | "journal" => app.journal_prompt()?,
            "t" | "tasks" | "task" => app.task_prompt()?,
            "n" | "notes" | "note" => app.note_prompt()?,
            "r" | "review" => app.review_prompt()?,
            "q" | "quit" => {
                println!();
                println!("    {} {}", green("✨"), bold("Goodbye!"));
                println!();
                break;
            }
            "" => {}
            _ => {
                println!("    {} Unknown command: {}", yellow("⚠"), input);
                println!("    {} Try: journal, tasks, notes, review, or quit", dim("→"));
                println!();
            }
        }
    }

    Ok(())
}
