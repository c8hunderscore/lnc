use std::env;
use std::path::PathBuf;
use std::fs;

enum FileType {
    Dir,
    File,
    Unknown,
}

fn main() {
    let mut args: Vec<String> = env::args()
        .collect();

    let program = args.remove(0);

    let mut files: Vec<String> = vec![];
    let mut lines: usize = 0;

    for file in args {
        if !exists(&file) {
            eprintln!("{}: no such file or directory: {}", program, file);
            continue;
        }

        check_file(&file, &mut files);
    }

    for file in files {
        let data = fs::read(&file).unwrap();

        for byte in data {
            if byte == 0xA {
                lines += 1;
            }
        }
    }

    println!("{}", lines);
}

fn check_file(file: &String, mut files: &mut Vec<String>) {
    let mut args: Vec<String> = env::args()
        .collect();

    let program = args.remove(0);

    match file_type(&file) {
        FileType::Dir => {
            for file in read_dir(&file) {
                check_file(&file, &mut files);
            }
        },
        FileType::File => files.push(file.to_string()),
        _ => eprintln!("{}: failed to resolve: {}", program, file),
    }
}

fn read_dir(file: &String) -> Vec<String> {
    fs::read_dir(file)
        .unwrap()
        .map(|f| f.unwrap()
             .path()
             .to_str()
             .unwrap()
             .to_string())
        .collect()
}

fn exists(file: &String) -> bool {
    PathBuf::from(file).exists()
}

fn file_type(file: &String) -> FileType {
    if PathBuf::from(&file).is_file() {
        return FileType::File;
    }

    if PathBuf::from(&file).is_dir() {
        return FileType::Dir;
    }

    FileType::Unknown
}

