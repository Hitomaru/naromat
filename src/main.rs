extern crate clap;

use clap::{App, Arg};
use naromat::entities::text_file::TextFile;
use naromat::errors::*;
use std::path::Path;

fn main() {
    let app = App::new("naromat")
        .version("0.2.0")
        .author("Hitomaru Horino <type10tk@kahi-sv.info>")
        .about("Text file formatter for Syosetsuka ni naro")
        .arg(Arg::with_name("source").help("source text file path").required(true))
        .arg(
            Arg::with_name("target")
                .help("file path to save")
                .short("t")
                .long("target")
                .takes_value(true),
        );
    let matches = app.get_matches();
    let source = matches.value_of("source").unwrap_or("./");
    let source = Path::new(source);
    let target = matches.value_of("target").unwrap_or("./");
    if source.is_file() {
        match process_file(source, target) {
            Ok(_) => 0,
            Err(_) => 1,
        };
    } else {
        match process_dir(source, target) {
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

fn stringify_path<'file_process>(path: &Path) -> Result<&str, InvalidPathError> {
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
