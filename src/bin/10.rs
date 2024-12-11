// Day 10: Hoof It

use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let input = rust_advent::read_number_grid("10")?;
    println!("Number of trails by distinct endpoint: {}", count_trails_by_distinct_endpoint(&input));
    println!("Number of distinct trails: {}", count_distinct_trails(&input));
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

// Counts the number of trails with distinct endpoints.
//
// A trail starts at height 0 and ends at height 9,
// and must increase by 1 in height at each step.
fn count_trails_by_distinct_endpoint(grid: &[Vec<u8>]) -> u32 {
    let mut n_trails = 0;

    let n_rows = grid.len();
    let n_cols = grid[0].len();
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().cloned().enumerate() {
            if cell != 0 {
                // Doesn't start a trail.
                continue;
            }
            let mut visited = HashSet::new();
            let mut stack = Vec::new();

            // Push the first points.
            if row_idx > 0
                && grid[row_idx - 1][col_idx] == 1
            {
                stack.push(Point {
                    x: row_idx - 1,
                    y: col_idx,
                });
                visited.insert(Point {
                    x: row_idx - 1,
                    y: col_idx,
                });
            }
            if col_idx > 0
                && grid[row_idx][col_idx - 1] == 1
            {
                stack.push(Point {
                    x: row_idx,
                    y: col_idx - 1,
                });
                visited.insert(Point {
                    x: row_idx,
                    y: col_idx - 1,
                });
            }
            if row_idx < n_rows - 1
                && grid[row_idx + 1][col_idx] == 1
            {
                stack.push(Point {
                    x: row_idx + 1,
                    y: col_idx,
                });
                visited.insert(Point {
                    x: row_idx + 1,
                    y: col_idx,
                });
            }
            if col_idx < n_cols - 1
                && grid[row_idx][col_idx + 1] == 1
            {
                stack.push(Point {
                    x: row_idx,
                    y: col_idx + 1,
                });
                visited.insert(Point {
                    x: row_idx,
                    y: col_idx + 1,
                });
            }

            while let Some(point) = stack.pop() {
                let curr_value = grid[point.x][point.y];
                if curr_value == 9 {
                    // Found the end of a trail.
                    // No need to continue the search from this point.
                    n_trails += 1;
                    continue;
                }
                // Push the next points.
                if point.x > 0
                    && grid[point.x - 1][point.y] == curr_value + 1
                    && !visited.contains(&Point {
                        x: point.x - 1,
                        y: point.y,
                    })
                {
                    let p = Point { x: point.x - 1, y: point.y };
                    stack.push(p);
                    visited.insert(p);
                }
                if point.y > 0
                    && grid[point.x][point.y - 1] == curr_value + 1
                    && !visited.contains(&Point {
                        x: point.x,
                        y: point.y - 1,
                    })
                {
                    let p = Point { x: point.x, y: point.y - 1 };
                    stack.push(p);
                    visited.insert(p);
                }
                if point.x < n_rows - 1
                    && grid[point.x + 1][point.y] == curr_value + 1
                    && !visited.contains(&Point {
                        x: point.x + 1,
                        y: point.y,
                    })
                {
                    let p = Point { x: point.x + 1, y: point.y };
                    stack.push(p);
                    visited.insert(p);
                }
                if point.y < n_cols - 1
                    && grid[point.x][point.y + 1] == curr_value + 1
                    && !visited.contains(&Point {
                        x: point.x,
                        y: point.y + 1,
                    })
                {
                    let p = Point { x: point.x, y: point.y + 1 };
                    stack.push(p);
                    visited.insert(p);
                }
            }
        }
    }
    n_trails
}

// Counts the number of distinct trails.
//
// A trail starts at height 0 and ends at height 9,
// and must increase by 1 in height at each step.
fn count_distinct_trails(grid: &[Vec<u8>]) -> u32 {
    let mut n_trails = 0;

    // This is like the previous solution, but we don't need to track visited points.
    // Since every trail starts at a different point, it is distinct from all other
    // trails.
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().cloned().enumerate() {
            if cell != 0 {
                // Doesn't start a trail.
                continue;
            }
            let mut stack = Vec::new();

            // Push the first points.
            if row_idx > 0
                && grid[row_idx - 1][col_idx] == 1
            {
                stack.push(Point {
                    x: row_idx - 1,
                    y: col_idx,
                });
            }
            if col_idx > 0
                && grid[row_idx][col_idx - 1] == 1
            {
                stack.push(Point {
                    x: row_idx,
                    y: col_idx - 1,
                });
            }
            if row_idx < n_rows - 1
                && grid[row_idx + 1][col_idx] == 1
            {
                stack.push(Point {
                    x: row_idx + 1,
                    y: col_idx,
                });
            }
            if col_idx < n_cols - 1
                && grid[row_idx][col_idx + 1] == 1
            {
                stack.push(Point {
                    x: row_idx,
                    y: col_idx + 1,
                });
            }

            while let Some(point) = stack.pop() {
                let curr_value = grid[point.x][point.y];
                if curr_value == 9 {
                    // Found the end of a trail.
                    // No need to continue the search from this point.
                    n_trails += 1;
                    continue;
                }
                // Push the next points.
                if point.x > 0
                    && grid[point.x - 1][point.y] == curr_value + 1
                {
                    let p = Point { x: point.x - 1, y: point.y };
                    stack.push(p);
                }
                if point.y > 0
                    && grid[point.x][point.y - 1] == curr_value + 1
                {
                    let p = Point { x: point.x, y: point.y - 1 };
                    stack.push(p);
                }
                if point.x < n_rows - 1
                    && grid[point.x + 1][point.y] == curr_value + 1
                {
                    let p = Point { x: point.x + 1, y: point.y };
                    stack.push(p);
                }
                if point.y < n_cols - 1
                    && grid[point.x][point.y + 1] == curr_value + 1
                {
                    let p = Point { x: point.x, y: point.y + 1 };
                    stack.push(p);
                }
            }
        }
    }
    n_trails
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_trails_by_distinct_endpoint_small() {
        let grid_str = r#"
            0123
            1234
            2345
            9876"#;
        let grid = rust_advent::parse_to_number_grid(grid_str);
        assert_eq!(count_trails_by_distinct_endpoint(&grid), 1);
    }

    #[test]
    fn test_count_trails_by_distinct_endpoint() {
        let grid_str = r#"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732"#;
        let grid = rust_advent::parse_to_number_grid(grid_str);

        assert_eq!(count_trails_by_distinct_endpoint(&grid), 36);
    }

    #[test]
    fn test_count_distinct_trails() {
        let grid_str = r#"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732"#;
        let grid = rust_advent::parse_to_number_grid(grid_str);

        assert_eq!(count_distinct_trails(&grid), 81);
    }
}   
