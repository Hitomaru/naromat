#[macro_use]
extern crate clap;

use clap::Arg;
use naromat::entities::text_file::TextFile;
use naromat::errors::*;
use std::path::Path;

fn main() {
    let app = app_from_crate!()
        .arg(Arg::with_name("source").help("source text file path").required(true))
        .arg(
            Arg::with_name("dest")
                .help("file/dir path to save")
                .short("d")
                .long("dest")
                .takes_value(true),
        );
    let matches = app.get_matches();
    let source = matches.value_of("source").unwrap_or("./");
    let source = Path::new(source);
    let dest = matches.value_of("dest").unwrap_or("./");
    if source.is_file() {
        match process_file(source, dest) {
            Ok(_) => 0,
            Err(_) => 1,
        };
    } else {
        match process_dir(source, dest) {
            Ok(_) => 0,
            Err(_) => 1,
        };
    }
}

fn process_file<'file_process>(
    source: &Path,
    target: &'file_process str,
) -> Result<(), FileProcessError<'file_process>> {
    print!("{} : processing", source.display());
    match TextFile::new(stringify_path(source).unwrap()) {
        Ok(file) => {
            print!("...loaded");
            match file.format_and_save(target) {
                Ok(_) => {
                    println!("..successfuly saved to {}", target);
                    Ok(())
                }
                Err(err) => {
                    println!("cannot be saved: {}", err);
                    Err(FileProcessError::SaveError(err))
                }
            }
        }
        Err(err) => {
            println!("cannot be loaded: {}", err);
            Err(FileProcessError::LoadError(err))
        }
    }
}

fn stringify_path(path: &Path) -> Result<&str, InvalidPathError> {
    match path.to_str() {
        Some(string) => Ok(string),
        None => Err(InvalidPathError::FileNotFound(path)),
    }
}

fn process_dir(source: &Path, target: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("{} : processing", source.display());
    for entry in source.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            process_dir(&path, target)?
        } else {
            let parent_dir = path.parent().unwrap().to_str().unwrap();
            let file_name = path.file_name().unwrap().to_str().expect("File name cannot be parsed");
            let target_dir = format!("{}/{}", target, parent_dir);
            std::fs::create_dir_all(&target_dir)?;
            let file_name = format!("{}/{}", target_dir, file_name);
            process_file(&path, file_name.as_str()).unwrap();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn can_process_recursively() {
        // given
        let source_dir = "./resources/test/main/can_process_recursively";
        let target_dir = "./resource/test/main/temp";
        let target_dir_path = Path::new(target_dir);
        let source_dir_path = Path::new(source_dir);
        let source_file_count = source_dir_path.ancestors().count();

        // when
        process_dir(source_dir_path, target_dir).unwrap();

        // then
        let target_file_count = target_dir_path.ancestors().count();

        // teardown
        remove_dir_all(target_dir).unwrap();

        // assert
        assert_eq!(target_file_count, source_file_count)
    }

    #[test]
    fn can_process_a_file() {
        // given
        let source_file = "./resources/test/main/can_process_a_file/source.txt";
        let target_file = "./resources/test/main/can_process_a_file/temp.txt";
        let target_file_path = Path::new(target_file);
        let source_file_path = Path::new(source_file);

        // when
        process_file(source_file_path, target_file).unwrap();

        // then
        let is_processed_file_exists = target_file_path.exists();

        // teardown
        remove_file(target_file_path).unwrap();

        // assert
        assert!(is_processed_file_exists)
    }
}
