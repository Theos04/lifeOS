use std::io::{self, Write};

// Color helpers
fn color(c: &str, text: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", c, text)
}

fn green(text: &str) -> String {
    color("32", text)
}

fn cyan(text: &str) -> String {
    color("36", text)
}

fn yellow(text: &str) -> String {
    color("33", text)
}

fn red(text: &str) -> String {
    color("31", text)
}

fn bold(text: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", text)
}

fn dim(text: &str) -> String {
    format!("\x1b[2m{}\x1b[0m", text)
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
                println!("    {}", dim("  No entries yet."));
                println!();
            }
            "n" | "notes" => {
                println!();
                println!("    {} {}", green("📓"), bold("Notes"));
                println!("    {}", dim("  No notes yet."));
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
