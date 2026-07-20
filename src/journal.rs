// A Journal preserves the moment before memory rewrites it.

#[derive(Debug, Clone, PartialEq)]
pub struct JournalEntry {
    date: String,
    content: String,
    mood: Option<String>,
    question: Option<String>,
}

impl JournalEntry {
    pub fn new(date: &str, content: &str) -> Self {
        Self {
            date: date.to_string(),
            content: content.to_string(),
            mood: None,
            question: None,
        }
    }

    pub fn with_mood(mut self, mood: &str) -> Self {
        self.mood = Some(mood.to_string());
        self
    }

    pub fn with_question(mut self, question: &str) -> Self {
        self.question = Some(question.to_string());
        self
    }

    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn mood(&self) -> Option<&str> {
        self.mood.as_deref()
    }

    pub fn question(&self) -> Option<&str> {
        self.question.as_deref()
    }

    // --- Representation ---

    pub fn to_record(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("Date: {}", self.date));
        lines.push(format!("Content: {}", self.content));
        if let Some(mood) = &self.mood {
            lines.push(format!("Mood: {}", mood));
        }
        if let Some(question) = &self.question {
            lines.push(format!("Question: {}", question));
        }
        lines
    }

    pub fn from_record(lines: &[String]) -> Result<Self, &'static str> {
        let mut date = None;
        let mut content = None;
        let mut mood = None;
        let mut question = None;

        for line in lines {
            if let Some(stripped) = line.strip_prefix("Date: ") {
                date = Some(stripped.to_string());
            } else if let Some(stripped) = line.strip_prefix("Content: ") {
                content = Some(stripped.to_string());
            } else if let Some(stripped) = line.strip_prefix("Mood: ") {
                mood = Some(stripped.to_string());
            } else if let Some(stripped) = line.strip_prefix("Question: ") {
                question = Some(stripped.to_string());
            }
        }

        let date = date.ok_or("Missing date")?;
        let content = content.ok_or("Missing content")?;

        let mut entry = JournalEntry::new(&date, &content);
        if let Some(m) = mood {
            entry = entry.with_mood(&m);
        }
        if let Some(q) = question {
            entry = entry.with_question(&q);
        }
        Ok(entry)
    }
}
