extern crate clap;

use clap::{App, Arg};
use naromat::entities::text_file::TextFile;
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
    let source_path = Path::new(source);
    let target = matches.value_of("target").unwrap_or("./");
    if source_path.is_file() {
        process_file(source, target);
    } else {
        process_dir(source_path).unwrap();
    }
}

fn process_file(source: &str, target: &str) {
    match TextFile::new(source) {
        Ok(file) => {
            println!("file {} loaded", source);
            match file.format_and_save(target) {
                Ok(_) => {
                    println!("file '{}' successfuly saved", target);
                }
                Err(err) => {
                    println!("file '{}' cannot be saved: {}", target, err);
                }
            };
        }
        Err(err) => {
            println!("the file '{}' cannot be loaded: {}", source, err);
        }
    };
}
fn process_dir(source: &Path) -> Result<(), Box<dyn std::error::Error + 'static>> {
    for entry in source.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            process_dir(&path)?
        } else {
            let parent_dir = path.parent().unwrap().to_str().unwrap();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_name = format!(
                "{}/formatted_{}",
                parent_dir,
                file_name
            );
            TextFile::new(path.to_str().unwrap())?.format_and_save(file_name.as_str()).unwrap();
        }
    }
    Ok(())
}
