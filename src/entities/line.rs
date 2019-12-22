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
    /// Line::new("我が輩は猫である。名前はまだない。");
    /// ```
    pub fn new(text : &str) -> Self {
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
    /// assert_eq!(line.get(), "　我が|輩《・》は|猫《ねこ》である。どこで生まれたかとんと見当がつかぬ。");
    /// ```
    /// 
    pub fn get(self) -> String {
        self.elements.into_iter().map(|sentence| sentence.get()).collect()
    }

    /// Format line
    fn format(text : &str) -> Self {
        let line = Self::add_header_space(text.trim());
        let sentences = Sentence::from_line(&line);
        Self { elements: sentences }
    }

    /// Insert 2 byte whitespace to line head
    fn add_header_space(text : &str) -> String {
        if Self::is_speech(text) { return " ".to_string() + text; }
        return "　".to_string() + text;
    }

    /// Generate multiple lines from multiline string
    /// 
    /// # Example
    /// 
    /// ```
    /// use naromat::entities::line::Line;
    /// let lines = Line::from_chapter("我が輩は猫である。
    /// 名前はまだない。
    /// どこで生まれたのかとんと見当が付かぬ。");
    /// 
    /// ```
    pub fn from_chapter(text : &str) -> Vec<Self> {
        text.split_terminator('\n').map(|line| { Self::new(line) }).collect()
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
#[cfg(test)]
mod tests {
    use super::Line;

    #[test]
    fn get() {
        let source   = "我が[輩:.]は[猫:ねこ]である。どこで生まれたかとんと見当がつかぬ。";
        let expected = "　我が|輩《・》は|猫《ねこ》である。どこで生まれたかとんと見当がつかぬ。";
        let line = Line::new(&source);
        assert_eq!(line.get(), expected);
    }

    #[test]
    fn from_chapter() {
        let source = "我が輩は猫である。
名前はまだない。
どこで生まれたのかとんと見当が付かぬ。";
        Line::from_chapter(source);
    }
}
