// Day 8: Resonant Collinearity

use std::collections::{HashMap, HashSet};

fn main() -> std::io::Result<()> {
    let input = rust_advent::read_file_as_string("08")?;

    let arrays = create_arrays(&input);

    println!(
        "Number of unique antinodes: {:?}",
        get_single_antinodes(&arrays).len()
    );
    println!(
        "Number of unique antinodes with multiples: {:?}",
        get_multiple_antinodes(&arrays).len()
    );
 
    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    column: i32,
}

impl std::ops::Add<Point> for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}

impl std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            column: self.column - other.column,
        }
    }
}

impl std::ops::SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

struct ArrayGrid {
    height: usize,
    width: usize,
    arrays: HashMap<char, Vec<Point>>,
}

impl ArrayGrid {
    fn is_in_bounds(&self, point: Point) -> bool {
        point.row >= 0
            && point.row < self.height as i32
            && point.column >= 0
            && point.column < self.width as i32
    }
}

fn create_arrays(input: &str) -> ArrayGrid {
    let mut arrays = HashMap::<char, Vec<Point>>::new();
    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    arrays.entry(c).or_default().push(Point {
                        row: row as i32,
                        column: column as i32,
                    });
                }
                _ => panic!("Unexpected character: {}", c),
            }
        }
    }
    ArrayGrid {
        height: input.lines().count(),
        width: input.lines().next().unwrap().len(),
        arrays,
    }
}

fn get_single_antinodes(grid: &ArrayGrid) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    for nodes_for_char in grid.arrays.values() {
        for first_idx in 0..(nodes_for_char.len() - 1) {
            let first = nodes_for_char[first_idx];
            for second_idx in (first_idx + 1)..nodes_for_char.len() {
                let second = nodes_for_char[second_idx];
                let delta = second - first;

                let first_antinode = second + delta;
                if grid.is_in_bounds(first_antinode) {
                    antinodes.insert(first_antinode);
                }

                let second_antinode = first - delta;
                if grid.is_in_bounds(second_antinode) {
                    antinodes.insert(second_antinode);
                }
            }
        }
    }
    antinodes
}

fn get_multiple_antinodes(grid: &ArrayGrid) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    for nodes_for_char in grid.arrays.values() {
        for first_idx in 0..(nodes_for_char.len() - 1) {
            let first = nodes_for_char[first_idx];
            for second_idx in (first_idx + 1)..nodes_for_char.len() {
                // Each antenna is an antinode as long as there is more
                // than one antenna with the same key.
                antinodes.insert(first);
                let second = nodes_for_char[second_idx];
                antinodes.insert(second);

                let delta = second - first;

                let mut first_antinode = second + delta;
                while grid.is_in_bounds(first_antinode) {
                        antinodes.insert(first_antinode);
                        first_antinode += delta;
                }

                let mut second_antinode = first - delta;
                while grid.is_in_bounds(second_antinode) {
                    antinodes.insert(second_antinode);
                    second_antinode -= delta;
                }
            }
        }
    }
    antinodes
}
