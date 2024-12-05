use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

const INPUT_BASE_PATH: &str = "/Users/alexconley/Programming/Advent Of Code/2024/input";

fn get_input_path(day: &str) -> PathBuf {
    let mut path = Path::new(INPUT_BASE_PATH).join(day);
    path.set_extension("txt");
    path
}

pub fn read_file_as_string(day: &str) -> std::io::Result<String> {
    std::fs::read_to_string(get_input_path(day))
}
 
pub fn read_int_pairs(day: &str) -> std::io::Result<(Vec<i32>, Vec<i32>)> {
    let reader = BufReader::new(File::open(get_input_path(day))?);
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        v1.push(
            parts
                .next()
                .expect("No first number")
                .parse()
                .expect("First number is not an integer"),
        );
        v2.push(
            parts
                .next()
                .expect("No second number")
                .parse()
                .expect("Second number is not an integer"),
        );
    }
    Ok((v1, v2))
}

pub fn read_number_grid(day: &str) -> std::io::Result<Vec<Vec<i32>>> {
    BufReader::new(File::open(get_input_path(day))?)
        .lines()
        .map(|line| {
            Ok(line?
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Value is not an i32"))
                .collect::<Vec<i32>>())
        })
        .collect()
}

pub fn read_ascii_grid(day: &str) -> std::io::Result<Vec<Vec<u8>>> {
    BufReader::new(File::open(get_input_path(day))?)
        .lines()
        .map(|line| Ok(line?.as_bytes().to_vec()))
        .collect()
}

