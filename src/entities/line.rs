use crate::entities::sentence::Sentence;
use regex::Regex;

/// Structure of novel line
///
/// Lines are defined below:
/// * Starts after breakline or chapter head
/// * End with breakline
pub struct Line {
    elements: Vec<Sentence>,
}

/// Implementation of novel line structure
impl Line {
    /// Constructor
    ///
    /// # Example
    ///
    /// ```
    /// use naromat::entities::line::Line;
    ///
    /// Line::new("我が輩は猫である。名前はまだない。");
    /// ```
    pub fn new(text: &str) -> Self {
        Self::format(text)
    }

    /// Print formatted line
    ///
    /// # Example
    ///
    /// ```
    /// use naromat::entities::line::Line;
    ///
    /// let line = Line::new("我が輩は猫である。名前はまだない。");
    /// line.print()
    /// ```
    pub fn print(self) {
        for element in self.elements {
            element.print();
        }
    }

    /// Get string of formatted sentence
    ///
    /// # Example
    ///
    /// ```
    /// use naromat::entities::line::Line;
    /// let line = Line::new("我が[輩:.]は[猫:ねこ]である。どこで生まれたかとんと見当がつかぬ。");
    /// assert_eq!(line.get(), "　我が｜輩《・》は｜猫《ねこ》である。どこで生まれたかとんと見当がつかぬ。");
    /// ```
    ///
    pub fn get(self) -> String {
        self.elements.into_iter().map(|sentence| sentence.get()).collect()
    }

    /// Format line
    fn format(text: &str) -> Self {
        let line = Self::add_header_space(text.trim());
        let line = Self::split(&line)
            .into_iter()
            .map(Sentence::new)
            .collect();
        Self { elements: line }
    }

    /// Insert 2 byte whitespace to line head
    fn add_header_space(text: &str) -> String {
        if Self::is_speech(text) {
            return " ".to_string() + text;
        }
        "　".to_string() + text
    }

    /// Split line to sentences
    fn split(text: &str) -> Vec<&str> {
        let sentence_terminators = Regex::new(r".*([」。.？！]|!\?|\?!|\z)").unwrap();
        sentence_terminators.find_iter(text).map(|m| m.as_str()).collect()
    }

    /// Return true if a line is speech line
    fn is_speech(text: &str) -> bool {
        let line_head = text.chars().next().unwrap_or(' ');
        matches!(line_head, '「')
    }

    pub fn is_comment(text: &str) -> bool {
        let line_head: String = text.trim().chars().take(2).collect();
        let comment_headers = vec!["//", ">", "#"];
        comment_headers.iter().any(|header| line_head.starts_with(header))
    }
}
#[cfg(test)]
mod tests {
    use super::Line;

    #[test]
    fn get() {
        let source = "我が[輩:.]は[猫:ねこ]である。どこで生まれたかとんと見当がつかぬ。";
        let expected = "　我が｜輩《・》は｜猫《ねこ》である。どこで生まれたかとんと見当がつかぬ。";
        let line = Line::new(source);
        assert_eq!(line.get(), expected);
    }

    #[test]
    fn get_min() {
        let source = "我";
        let expected = "　我";
        let line = Line::new(source);
        assert_eq!(line.get(), expected);
    }

    #[test]
    fn is_comment_should_return_false_when_body_string() {
        assert!(!Line::is_comment("吾輩は猫である"));
    }

    #[test]
    fn is_comment_should_return_true_when_comment_string() {
        assert!(Line::is_comment("// 吾輩は猫である"));
    }

    #[test]
    fn is_comment_should_return_true_when_half_spaced_comment_string() {
        assert!(Line::is_comment(" // 吾輩は猫である"));
    }

    #[test]
    fn is_comment_should_return_true_when_full_spaced_comment_string() {
        assert!(Line::is_comment("　// 吾輩は猫である"));
    }

    #[test]
    fn is_comment_should_return_true_when_multi_half_spaced_comment_string() {
        assert!(Line::is_comment("  // 吾輩は猫である"));
    }

    #[test]
    fn is_comment_should_return_true_when_multi_full_spaced_comment_string() {
        assert!(Line::is_comment("　　// 吾輩は猫である"));
    }
}
