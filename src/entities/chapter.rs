use crate::entities::line::Line;

pub struct Chapter {
    lines : Vec<Line>,
}

impl Chapter {
    pub fn new(text : &str) -> Self {
        Self { lines: text.split_terminator('\n').map(|line| { Line::new(line) }).collect() }
    }

    pub fn print(self) {
        for line in self.lines {
            line.print();
        }
    }
}