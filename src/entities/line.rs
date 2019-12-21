use crate::entities::sentence::Sentence;
use regex::Regex;

/// Structure of novel line
/// 
/// Lines are defined below:
/// * Starts after breakline or chapter head
/// * End with breakline
pub struct Line {
    elements : Vec<Sentence>,
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
    /// Line::new("我が輩は猫である。名前はまだない。")
    /// ```
    pub fn new(text : &str) -> Self {
        Self { elements: Self::format(text) }
    }

    /// Print formatted line
    /// 
    /// # Example
    /// 
    /// ```
    /// use naromat::entities::line::Line;
    /// 
    /// let line = Line::new("我が輩は猫である。名前はまだない。");
    /// sentence.print()
    /// ```
    pub fn print(self) {
        for element in self.elements {
            element.print();
        }
    }

    /// Format line
    fn format(text : &str) -> Vec<Sentence> {
        let line = Self::add_header_space(text.trim());
        Self::split(&line).into_iter().map(|sentence| Sentence::new(sentence)).collect()
    }

    /// Insert 2 byte whitespace to line head
    fn add_header_space(text : &str) -> String {
        if Self::is_speech(text) { return " ".to_string() + text; }
        return "　".to_string() + text;
    }

    /// Split line to sentences
    fn split(text : &str) -> Vec<&str> {
        let line_terminators = Regex::new(r".*([」。.？！]|!\?|\?!)").unwrap();
        line_terminators.find_iter(text).map(|m| m.as_str()).collect()
    }

    /// Return true if a line is speech line
    fn is_speech(text : &str) -> bool {
        let line_head = text.chars().nth(0).unwrap_or(' ');
         match line_head {
             '「' => true,
             _    => false
         }
    }
}