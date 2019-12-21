use regex::Regex;
use regex::Captures;

/// Structure of novel sentence.
/// 
/// Sentences are defined below:
/// * End with sentence-terminators ('.', ''))
/// 
pub struct Sentence {
    elements : String,
}

/// Implementation for novel sentence structure
impl Sentence {
    /// Constructor
    /// 
    /// # Example
    /// 
    /// ```
    /// use crate::entities::sentence::Sentence
    /// 
    /// Sentence::new("我が輩は猫である。")
    /// ```
    pub fn new(sentence : &str) -> Self {
        Self { elements: sentence.trim_end().to_string() }
    }

    /// Print formatted sentence
    /// 
    /// # Example
    /// 
    /// ```
    /// use crate::entities::sentence::Sentence
    /// 
    /// let sentence = Sentence::new("我が輩は猫である。")
    /// sentence.print()
    /// ```
    pub fn print(self) {
        println!("{}", self.format().elements)
    }

    /// Get string of formatted sentence
    /// 
    /// # Example
    /// 
    /// ```
    /// use crate::entities::sentence::Sentence
    /// let sentence  Sentence::new("我が[輩:.]は[猫:ねこ]である")
    /// assert_eq!(sentence.get(), "我が|輩《・》は|猫《ねこ》である")
    /// ```
    /// 
    pub fn get(self) -> String {
        self.format().elements.to_string()
    }

    fn format(&self) -> Self {
        self.add_space_after_exclamation().convert_kenten().convert_ruby()
    }

    fn add_space_after_exclamation(&self) -> Self {
        let exclamations = Regex::new(r"(!\?|\?!|[！？])").unwrap();
        let sentence = exclamations.replace_all(&self.elements, "$1　").to_string();
        Self::new(&sentence)
    }
    fn convert_kenten(&self) -> Self {
        let kenten = Regex::new(r"\[(.*?):\.\]").unwrap();
        let sentence = kenten.replace_all(&self.elements, |caps: &Captures| {
            format!(
                "|{}《{}》",
                &caps[1],
                "・".to_string().repeat(caps[1].chars().count()
            ))
        }).to_string();
        Self::new(&sentence)
    }
    fn convert_ruby(&self) -> Self {
        let ruby = Regex::new(r"\[(.*?):(.*?)]").unwrap();
        let sentence = ruby.replace_all(&self.elements, "|$1《$2》").to_string();
        Self::new(&sentence)
    }
}

#[cfg(test)]
mod tests {
    use super::Sentence;

    #[test]
    fn convert_ruby_should_convert_ruby() {
        let sut = Sentence::new("私の[名前:なまえ]は[太郎:たろう]です");
        let expected = "私の|名前《なまえ》は|太郎《たろう》です";
        assert_eq!(sut.convert_ruby().elements, expected);  
    }

    #[test]
    fn convert_kenten_should_convert_kenten() {
        let sut = Sentence::new("私の[名前:.]は[たろう:.]です");
        let expected = "私の|名前《・・》は|たろう《・・・》です";
        assert_eq!(sut.convert_kenten().elements, expected);  
    }

}
