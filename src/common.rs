use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn write_conf(file_name: String, conf: Vec<String>){
    fs::write(file_name, conf.join("\n")).expect("");
}

pub fn get_home_dir() -> String {
    let homedir = env::var("HOME").expect("$HOME is not set");
    homedir
}