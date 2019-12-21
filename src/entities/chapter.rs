use crate::entities::line::Line;

/// Structure of novel chapter.
/// 
/// Chapters are defined below:
/// * Starts from previous chapter or start of document
/// * End with next chapter or end of document
/// 
pub struct Chapter {
    lines : Vec<Line>,
}

/// Implementation for novel chapter structure
impl Chapter {
    /// Constructor
    /// 
    /// # Example
    /// 
    /// ```
    /// use naromat::entities::chapter::Chapter;
    /// 
    /// Chapter::new("
    /// 我が輩は猫である。名前はまだない。
    /// どこで生まれたのかとんと検討がつかぬ。");
    /// ```
    pub fn new(text : &str) -> Self {
        Self { lines: text.split_terminator('\n').map(|line| { Line::new(line) }).collect() }
    }

    pub fn print(self) {
        for line in self.lines {
            line.print();
        }
    }
}