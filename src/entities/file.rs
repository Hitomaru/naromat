use crate::entities::chapter::Chapter;
use std::fs;
use std::fs::File;
use std::io::{ Write };

/// Structure of Novel text file
pub struct TextFile {
    chapter : Chapter,
}

/// Implementation for novel text structure
impl TextFile {

    /// Constructor
    ///
    /// # Fail
    /// 
    /// * Path format is invalid
    /// * A file designated by a path is not exists
    ///  
    /// # Example
    /// 
    /// ```no_run
    /// use naromat::entities::file::TextFile;
    /// 
    /// TextFile::new("./path/to/source/file");
    /// ```
    pub fn new(path_from : &str) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let text = fs::read_to_string(path_from)?;
        Ok(Self { chapter: Chapter::new(text.as_str()) })
    }

    /// Format file text and save
    /// 
    /// Return true if save was succeed
    /// 
    /// Return false if save was failed
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use naromat::entities::file::TextFile;
    /// 
    /// let text = TextFile::new("./path/to/source/file").unwrap();
    /// text.format_and_save("./path/to/save");
    /// ```
    pub fn format_and_save(self, path_to : &str) -> bool {
        let mut file = match File::create(path_to) {
            Ok(f)   => f,
            Err(_)  => return false
        };
        let result = match write!(file, "{}\n", self.chapter.get()) {
            Ok(_)   => true,
            Err(_)  => false, 
        };
        file.flush().unwrap();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::TextFile;
    use file_diff::{diff};
    use std::fs;

    #[test]
    fn format_and_save() {
        // given
        let source_file_path = "./resources/test/entities/file/source.txt";
        let reference_file_path = "./resources/test/entities/file/reference.txt";
        let target_file_path = "./resources/test/entities/file/target.txt";
        let target_file = TextFile::new(source_file_path).unwrap();

        
        // when
        target_file.format_and_save(target_file_path);

        // then
        let is_target_text_is_same_to_reference = diff(target_file_path, reference_file_path);
        
        // teardown
        fs::remove_file(target_file_path).unwrap();
        
        // assert
        assert!(is_target_text_is_same_to_reference);
    }
}
