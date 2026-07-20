// A Note is the trace of a thought in progress.

use chrono::Local;

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    fragment: String,
    created_at: String,
    context: Option<String>,
}

impl Note {
    pub fn new(fragment: &str) -> Self {
        Self {
            fragment: fragment.to_string(),
            created_at: Local::now().format("%Y-%m-%d %H:%M").to_string(),
            context: None,
        }
    }

    pub fn with_context(mut self, context: &str) -> Self {
        self.context = Some(context.to_string());
        self
    }

    pub fn with_created_at(mut self, created_at: &str) -> Self {
        self.created_at = created_at.to_string();
        self
    }

    pub fn fragment(&self) -> &str {
        &self.fragment
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }

    // --- Representation ---

    pub fn to_record(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("Fragment: {}", self.fragment));
        lines.push(format!("Created: {}", self.created_at));
        if let Some(context) = &self.context {
            lines.push(format!("Context: {}", context));
        }
        lines
    }

    pub fn from_record(lines: &[String]) -> Result<Self, &'static str> {
        let mut fragment = None;
        let mut created = None;
        let mut context = None;

        for line in lines {
            if let Some(stripped) = line.strip_prefix("Fragment: ") {
                fragment = Some(stripped.to_string());
            } else if let Some(stripped) = line.strip_prefix("Created: ") {
                created = Some(stripped.to_string());
            } else if let Some(stripped) = line.strip_prefix("Context: ") {
                context = Some(stripped.to_string());
            }
        }

        let fragment = fragment.ok_or("Missing fragment")?;

        let mut note = Note::new(&fragment);
        if let Some(c) = created {
            note = note.with_created_at(&c);
        }
        if let Some(ctx) = context {
            note = note.with_context(&ctx);
        }
        Ok(note)
    }
}
