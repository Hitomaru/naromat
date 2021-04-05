use crate::entities::line::Line;

/// Structure of novel chapter.
///
/// Chapters are defined below:
/// * Starts from previous chapter or start of document
/// * End with next chapter or end of document
///
pub struct Chapter {
    lines: Vec<Line>,
}

/// Implementation for novel chapter structure
impl Chapter {
    /**
    Constructor
    # Example
    ```
    use naromat::entities::chapter::Chapter;
    Chapter::new("
    我が輩は猫である。名前はまだない。
    どこで生まれたのかとんと検討がつかぬ。");
    ```
    */
    pub fn new(text: &str) -> Self {
        Self {
            lines: text.split_terminator('\n')
                .filter(|text| ! Line::is_comment(text))
                .map(|line| Line::new(line)).collect(),
        }
    }

    /// Print formatted chapter
    ///
    /// # Example
    ///
    /// ```
    /// use naromat::entities::chapter::Chapter;
    ///
    /// let chapter = Chapter::new("
    /// 我が輩は猫である。名前はまだない。
    /// どこで[生まれた:.]のかとんと[見当:けんとう]がつかぬ。
    /// ");
    /// chapter.print()
    /// ```
    pub fn print(self) {
        for line in self.lines {
            line.print();
        }
    }

    /**
    Get string of formatted sentence
    # Example
    ```
    use naromat::entities::chapter::Chapter;
    let chapter = Chapter::new("
    我が輩は猫である。名前はまだない。
    // コメント行
      // ネストされたコメント行
    どこで[生まれた:.]のかとんと[見当:けんとう]がつかぬ。
    ");
    assert_eq!(chapter.get(), "
    　我が輩は猫である。名前はまだない。
    　どこで｜生まれた《・・・・》のかとんと｜見当《けんとう》がつかぬ。");
    ```
    */
    pub fn get(self) -> String {
        let text: Vec<String> = self.lines.into_iter().map(|line| line.get()).collect();
        text.join("\n")
    }
}
#[cfg(test)]
mod tests {
    use super::Chapter;

    #[test]
    fn get() {
        let source = "我が輩は猫である。名前はまだない。
どこで[生まれた:.]のかとんと[見当:けんとう]がつかぬ。
// コメント行";
        let expected = "　我が輩は猫である。名前はまだない。
　どこで｜生まれた《・・・・》のかとんと｜見当《けんとう》がつかぬ。";
        let chapter = Chapter::new(&source);
        assert_eq!(chapter.get(), expected);
    }
}
