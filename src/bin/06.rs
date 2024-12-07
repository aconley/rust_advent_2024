use std::collections::HashSet;

// Day 6
fn main() -> std::io::Result<()> {
    let lines = rust_advent::read_file_as_lines("06")?;
    println!(
        "Squares covered by guard: {}",
        squares_covered_by_guard(&lines)
    );
    Ok(())
}

fn squares_covered_by_guard(lines: &Vec<String>) -> i32 {
    let mut obstacles = HashSet::<Point>::new();
    let mut guard = Guard {
        location: Point { x: 0, y: 0 },
        facing: Direction::Up,
    };

    // Numbering from upper left corner.
    let height = lines.len();
    let width = lines[0].len();
    for (row_idx, line) in lines.iter().enumerate() {
        assert_eq!(
            line.len(),
            width,
            "Row {} wrong length; expected {} got {}",
            row_idx,
            width,
            line.len()
        );
        for (col_idx, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    obstacles.insert(Point {
                        x: col_idx as i32,
                        y: row_idx as i32,
                    });
                }
                '^' => {
                    guard = Guard {
                        location: Point {
                            x: col_idx as i32,
                            y: row_idx as i32,
                        },
                        facing: Direction::Up,
                    }
                }
                '>' => {
                    guard = Guard {
                        location: Point {
                            x: col_idx as i32,
                            y: row_idx as i32,
                        },
                        facing: Direction::Right,
                    }
                }
                'v' => {
                    guard = Guard {
                        location: Point {
                            x: col_idx as i32,
                            y: row_idx as i32,
                        },
                        facing: Direction::Down,
                    }
                }
                '<' => {
                    guard = Guard {
                        location: Point {
                            x: col_idx as i32,
                            y: row_idx as i32,
                        },
                        facing: Direction::Left,
                    }
                }
                _ => (),
            }
        }
    }

    let mut visited = HashSet::<Point>::new();
    visited.insert(guard.location);
    loop {
        let next_point = guard.get_next_move();
        if next_point.x < 0 || next_point.x >= width as i32 || next_point.y < 0 || next_point.y >= height as i32 {  
            break;
        } else if obstacles.contains(&next_point) {
            guard.turn_right();
        } else {
            visited.insert(next_point);
            guard.location = next_point;
        }
    }
    visited.len() as i32
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct Guard {
    location: Point,
    facing: Direction,
}

impl Guard {
    fn get_next_move(&self) -> Point {
        match self.facing {
            Direction::Up => Point {
                x: self.location.x,
                y: self.location.y - 1,
            },
            Direction::Down => Point {
                x: self.location.x,
                y: self.location.y + 1,
            },
            Direction::Left => Point {
                x: self.location.x - 1,
                y: self.location.y,
            },
            Direction::Right => Point {
                x: self.location.x + 1,
                y: self.location.y,
            },
        }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }
}
