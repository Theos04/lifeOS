// Daily Review - a mirror, not an analysis.

#[derive(Debug, Clone)]
pub struct DailyReview {
    date: String,
    reflection: Option<String>,
}

impl DailyReview {
    pub fn new(date: &str) -> Self {
        Self {
            date: date.to_string(),
            reflection: None,
        }
    }

    pub fn with_reflection(mut self, reflection: &str) -> Self {
        self.reflection = Some(reflection.to_string());
        self
    }

    pub fn date(&self) -> &str {
        &self.date
    }

    pub fn reflection(&self) -> Option<&str> {
        self.reflection.as_deref()
    }

    // --- Representation ---

    pub fn to_record(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("Date: {}", self.date));
        if let Some(reflection) = &self.reflection {
            lines.push(format!("Reflection: {}", reflection));
        }
        lines
    }

    pub fn from_record(lines: &[String]) -> Result<Self, &'static str> {
        let mut date = None;
        let mut reflection = None;

        for line in lines {
            if let Some(stripped) = line.strip_prefix("Date: ") {
                date = Some(stripped.to_string());
            } else if let Some(stripped) = line.strip_prefix("Reflection: ") {
                reflection = Some(stripped.to_string());
            }
        }

        let date = date.ok_or("Missing date")?;

        let mut review = DailyReview::new(&date);
        if let Some(r) = reflection {
            review = review.with_reflection(&r);
        }
        Ok(review)
    }
}
