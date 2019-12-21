use crate::entities::sentence::Sentence;
use regex::Regex;

pub struct Line {
    elements : Vec<Sentence>,
}

impl Line {
    pub fn new(text : &str) -> Self {
        Self { elements: Self::format(text) }
    }

    pub fn print(self) {
        for element in self.elements {
            element.print();
        }
    }

    fn format(text : &str) -> Vec<Sentence> {
        let line = Self::add_header_space(text.trim());
        Self::split(&line).into_iter().map(|sentence| Sentence::new(sentence)).collect()
    }

    fn add_header_space(text : &str) -> String {
        if Self::is_speech(text) { return " ".to_string() + text; }
        return "　".to_string() + text;
    }

    fn split(text : &str) -> Vec<&str> {
        let line_terminators = Regex::new(r".*([」。.？！]|!\?|\?!)").unwrap();
        line_terminators.find_iter(text).map(|m| m.as_str()).collect()
    }

    fn is_speech(text : &str) -> bool {
        let line_head = text.chars().nth(0).unwrap_or(' ');
         match line_head {
             '「' => true,
             _    => false
         }
    }
}