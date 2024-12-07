use std::collections::HashSet;

// Day 6
fn main() -> std::io::Result<()> {
    let lines = rust_advent::read_file_as_lines("06")?;
    println!(
        "Squares covered by guard: {}",
        count_visited_squares(&lines)
    );
    println!(
        "Number of positions that would cause loop: {}",
        count_looping_obstacles(&lines)
    );
 
    Ok(())
}

fn count_visited_squares(lines: &Vec<String>) -> i32 {
    let grid = parse_grid(lines);
    squares_covered_by_guard(&grid).len() as i32
}

fn squares_covered_by_guard(grid: &Grid) -> HashSet<Point> {
    let mut visited = HashSet::<Point>::new();
    visited.insert(grid.guard_initial_position.location);
    let mut guard = grid.guard_initial_position;
    loop {
        let next_position = guard.get_next_move();
        if next_position.x < 0
            || next_position.x >= grid.width
            || next_position.y < 0
            || next_position.y >= grid.height
        {
            break;
        } else if grid.obstacles.contains(&next_position) {
            guard.turn_right();
        } else {
            visited.insert(next_position);
            guard.location = next_position;
        }
    }
    visited
}

fn count_looping_obstacles(lines: &Vec<String>) -> i32 {
    let mut grid = parse_grid(lines);
    let visited = squares_covered_by_guard(&grid);
    if visited.is_empty() {
        return 0;
    }

    let mut n_obstacles_that_cause_loop = 0;
    for position in visited {
        grid.obstacles.insert(position);
        if is_guard_in_loop(&grid) {
            n_obstacles_that_cause_loop += 1;
        }
        grid.obstacles.remove(&position);
    }
    n_obstacles_that_cause_loop
}

fn is_guard_in_loop(grid: &Grid) -> bool {
    // This time we need to track the direction as well.
    let mut visited = HashSet::<Guard>::new();
    visited.insert(grid.guard_initial_position);
    let mut guard = grid.guard_initial_position;
    loop {
        let next_position = guard.get_next_move();
        if next_position.x < 0
            || next_position.x >= grid.width
            || next_position.y < 0
            || next_position.y >= grid.height
        {
            // We exited the grid, the guard was definitely not stuck.
            return false;
        } else if grid.obstacles.contains(&next_position) {
            guard.turn_right();
        } else {
            if !visited.insert(Guard { location: next_position, facing: guard.facing }) {
                // Returned to previous state, the guard is in a loop.
                return true;
            }
            guard.location = next_position;
        }
    }
}

struct Grid {
    width: i32,
    height: i32,
    obstacles: HashSet<Point>,
    guard_initial_position: Guard,
}

fn parse_grid(lines: &Vec<String>) -> Grid {
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
    Grid {
        width: width as i32,
        height: height as i32,
        obstacles,
        guard_initial_position: guard,
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
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

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
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
