use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const INPUT_BASE_PATH: &str = "/Users/alexconley/Programming/Advent Of Code/2024/input";

pub fn read_int_pairs(day: &str) -> std::io::Result<(Vec<i32>, Vec<i32>)> {
    let mut path = Path::new(INPUT_BASE_PATH).join(day);
    path.set_extension("txt");
    let reader = BufReader::new(File::open(path)?);
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
