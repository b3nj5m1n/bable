#[macro_use]
extern crate clap;
use clap::ArgMatches;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

fn parse_args() -> ArgMatches<'static> {
    let matches = clap_app!(bable =>
        (version: "1.0")
        (author: "Benjamin <b3nj4m1n@gmx.net>")
        (about: "Rudimentary org-babel clone")
        (@arg LANGUAGE: -l --language +takes_value "Programming language to use for processing")
    ).get_matches();
    return matches;
}

fn get_program() -> String {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer).unwrap();
    return buffer;
}

fn prepare(language: &str, code: String, uuid: String) -> PathBuf {
    let path = Path::new("/")
        .join("tmp")
        .join("bable")
        .join(language)
        .join(uuid);
    fs::create_dir_all(&path).unwrap();
    fs::write(&path.join("code"), code).unwrap();
    return path;
}

fn process(language: &str, code: String) -> String {
    let uuid = Uuid::new_v4().to_string();
    let path = prepare(language, code, uuid);
    let output;
    let file_path = path.join("code").clone()
        .into_os_string().into_string().unwrap();
    let interpreter_name = match language {
        "python" => "python",
        _ => panic!("Language not recognised."),
    };
    output = Command::new(interpreter_name)
        .arg(file_path)
        .output()
        .expect("failed to execute process");
    return String::from_utf8_lossy(&output.stdout).to_string();
}

fn main() {
    let arguments = parse_args();
    let language = arguments.value_of("LANGUAGE").unwrap();
    let code = get_program();
    let result = process(language, code);
    println!("{}", result);
}
