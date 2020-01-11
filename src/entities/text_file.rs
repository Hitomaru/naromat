use crate::entities::chapter::Chapter;
use crate::errors::TextFileOutputError;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Structure of Novel text file
pub struct TextFile {
    chapter: Chapter,
}

/// Implementation for novel text structure
impl<'file_handling> TextFile {
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
    /// use naromat::entities::text_file::TextFile;
    ///
    /// TextFile::new("./path/to/source/file.txt").unwrap();
    /// ```
    pub fn new(path_from: &str) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let text = fs::read_to_string(path_from)?;
        Ok(Self {
            chapter: Chapter::new(text.as_str()),
        })
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
    /// use naromat::entities::text_file::TextFile;
    ///
    /// let text = TextFile::new("./path/to/source/file.txt").unwrap();
    /// text.format_and_save("./path/to/save.txt");
    /// ```
    pub fn format_and_save(self, path_to: &'file_handling str) -> Result<(), TextFileOutputError> {
        Self::touch_file(path_to).and_then(|file| self.save_file(file))
    }

    fn touch_file(path_to: &'file_handling str) -> Result<File, TextFileOutputError> {
        if Path::new(path_to).exists() {
            return Err(TextFileOutputError::AlreadyExists(path_to));
        }
        match File::create(path_to) {
            Ok(file) => Ok(file),
            Err(cause) => Err(TextFileOutputError::CannotCreate(cause)),
        }
    }

    fn save_file(self, mut file: File) -> Result<(), TextFileOutputError<'file_handling>> {
        let result = match write!(file, "{}\n", self.chapter.get()) {
            Ok(_) => Ok(()),
            Err(cause) => return Err(TextFileOutputError::CannotWrite(cause)),
        };
        match file.flush() {
            Ok(_) => result,
            Err(cause) => return Err(TextFileOutputError::CannotFlush(cause)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TextFile;
    use file_diff::diff;
    use std::fs;

    #[test]
    fn can_save_formatted_text() {
        // given
        let source_file_path = "./resources/test/entities/file/source.txt";
        let reference_file_path = "./resources/test/entities/file/reference.txt";
        let target_file_path = "./resources/test/entities/file/target.txt";
        let target_file = TextFile::new(source_file_path).unwrap();

        // when
        target_file.format_and_save(target_file_path).unwrap();

        // then
        let is_target_text_is_same_to_reference = diff(target_file_path, reference_file_path);
        // teardown
        fs::remove_file(target_file_path).unwrap();
        // assert
        assert!(is_target_text_is_same_to_reference);
    }

    #[test]
    fn return_error_when_target_file_already_exists() {
        // given
        let source_file_path = "./resources/test/entities/file/source.txt";
        let target_file_path = "./resources/test/entities/file/target.txt";
        let target_file = TextFile::new(source_file_path).unwrap();

        // expect
        let is_return_error = target_file.format_and_save(target_file_path).is_err();
        let is_target_file_not_modified = fs::metadata(target_file_path).unwrap().len() == 0;

        // assert
        assert!(is_return_error);
        assert!(is_target_file_not_modified);
    }

    #[test]
    fn return_error_when_source_file_does_not_exists() {
        // given
        let source_file_path = "./resources/test/entities/file/not/exists.txt";

        // expect
        let is_return_error = TextFile::new(source_file_path).is_err();

        // assert
        assert!(is_return_error);
    }

    #[test]
    fn return_error_when_source_file_path_is_invalid() {
        // given
        let source_file_path = "\\not.valid>/pat h@";

        // expect
        let is_return_error = TextFile::new(source_file_path).is_err();

        // assert
        assert!(is_return_error);
    }
}
