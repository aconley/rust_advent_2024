// Day 12: Garden Groups

use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let input = rust_advent::read_ascii_grid("12")?;
    println!("Compute fence cost {}", find_cost(&input));
    Ok(())
}

const DIRECTIONS: [(i16, i16); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    row: i16,
    col: i16,
}

struct Region {
    cells: HashSet<Point>,
}

impl Region {
    fn perimeter(&self) -> u16 {
        self.cells
            .iter()
            .map(|point| {
                DIRECTIONS
                    .iter()
                    .filter(|(dr, dc)| {
                        !self.cells.contains(&Point {
                            row: point.row + dr,
                            col: point.col + dc,
                        })
                    })
                    .count() as u16
            })
            .sum()
    }

    fn cost(&self) -> u32 {
        self.cells.len() as u32 * self.perimeter() as u32
    }

    fn exterior_edges(&self) -> HashSet<Segment> {
        fn get_edges(point: Point) -> [Segment; 4] {
            [
                Segment {
                    lower_left: point,
                    orientation: Orientation2D::Vertical,
                },
                Segment {
                    lower_left: Point {
                        row: point.row,
                        col: point.col + 1,
                    },
                    orientation: Orientation2D::Vertical,
                },
                Segment {
                    lower_left: Point {
                        row: point.row + 1,
                        col: point.col,
                    },
                    orientation: Orientation2D::Horizontal,
                },
                Segment {
                    lower_left: point,
                    orientation: Orientation2D::Horizontal,
                },
            ]
        }

        let mut edges = HashSet::new();
        for point in self.cells.iter() {
            for edge in get_edges(*point) {
                if !edges.insert(edge) {
                    // If the edge is already in the set, it's an interior edge and should be removed.
                    edges.remove(&edge);
                }
            }
        }
        edges
    }
}

fn find_regions(grid: &[Vec<u8>]) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut visited = HashSet::new();

    let max_row = grid.len() as i16;
    let max_col = grid[0].len() as i16;
    for (row, line) in grid.iter().enumerate() {
        for (col, &plant_type) in line.iter().enumerate() {
            let point = Point {
                row: row as i16,
                col: col as i16,
            };
            if visited.contains(&point) {
                continue;
            }

            // Start a new region.
            let mut region = Region {
                cells: HashSet::new(),
            };

            let mut stack = Vec::new();
            visited.insert(point);
            stack.push(point);
            region.cells.insert(point);

            while let Some(point) = stack.pop() {
                for (dr, dc) in DIRECTIONS {
                    let neighbor_point = Point {
                        row: point.row + dr,
                        col: point.col + dc,
                    };
                    if neighbor_point.row >= 0
                        && neighbor_point.col >= 0
                        && neighbor_point.row < max_row
                        && neighbor_point.col < max_col
                        && !visited.contains(&neighbor_point)
                        && grid[neighbor_point.row as usize][neighbor_point.col as usize]
                            == plant_type
                    {
                        visited.insert(neighbor_point);
                        region.cells.insert(neighbor_point);
                        stack.push(neighbor_point);
                    }
                }
            }
            regions.push(region);
        }
    }
    regions
}

fn find_cost(grid: &[Vec<u8>]) -> u32 {
    find_regions(grid).iter().map(|region| region.cost()).sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Orientation2D {
    Horizontal,
    Vertical,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Segment {
    lower_left: Point,
    orientation: Orientation2D,
}

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(input: Vec<&str>) -> Vec<Vec<u8>> {
        input.iter().map(|line| line.as_bytes().to_vec()).collect()
    }

    #[test]
    fn test_find_regions() {
        let test_input = vec!["AAAA", "BBCD", "BBCC", "EEEC"];
        assert_eq!(find_regions(&grid(test_input)).len(), 5);
    }

    #[test]
    fn test_find_regions_sizes() {
        let test_input = vec!["AAAA", "BBCD", "BBCC", "EEEC"];
        let regions = find_regions(&grid(test_input));
        assert_eq!(
            regions
                .iter()
                .map(|region| region.cells.len())
                .collect::<Vec<_>>(),
            vec![4, 4, 4, 1, 3]
        );
    }

    #[test]
    fn test_exterior_edges_single_cell() {
        let region = Region {
            cells: HashSet::from([Point { row: 1, col: 1 }]),
        };
        assert_eq!(
            region.exterior_edges(),
            HashSet::from([
                Segment {
                    lower_left: Point { row: 1, col: 1 },
                    orientation: Orientation2D::Vertical
                },
                Segment {
                    lower_left: Point { row: 1, col: 2 },
                    orientation: Orientation2D::Vertical
                },
                Segment {
                    lower_left: Point { row: 1, col: 1 },
                    orientation: Orientation2D::Horizontal
                },
                Segment {
                    lower_left: Point { row: 2, col: 1 },
                    orientation: Orientation2D::Horizontal
                }
            ])
        );
    }

    #[test]
    fn test_exterior_edges_l_shape() {
        let region = Region {
            cells: HashSet::from([
                Point { row: 0, col: 0 },
                Point { row: 0, col: 1 },
                Point { row: 1, col: 0 },
            ]),
        };
        assert_eq!(
            region.exterior_edges(),
            HashSet::from([
                Segment {
                    lower_left: Point { row: 0, col: 0 },
                    orientation: Orientation2D::Vertical
                },
                Segment {
                    lower_left: Point { row: 1, col: 0 },
                    orientation: Orientation2D::Vertical
                },
                Segment {
                    lower_left: Point { row: 2, col: 0 },
                    orientation: Orientation2D::Horizontal
                },
                Segment {
                    lower_left: Point { row: 1, col: 1 },
                    orientation: Orientation2D::Vertical
                },
                Segment {
                    lower_left: Point { row: 1, col: 1 },
                    orientation: Orientation2D::Horizontal
                },
                Segment {
                    lower_left: Point { row: 0, col: 2 },
                    orientation: Orientation2D::Vertical
                },
                Segment {
                    lower_left: Point { row: 0, col: 1 },
                    orientation: Orientation2D::Horizontal
                },
                Segment {
                    lower_left: Point { row: 0, col: 0 },
                    orientation: Orientation2D::Horizontal
                },
            ])
        );
    }

    #[test]
    fn test_find_cost_small() {
        let test_input = vec!["AAAA", "BBCD", "BBCC", "EEEC"];
        assert_eq!(find_cost(&grid(test_input)), 140);
    }

    #[test]
    fn test_find_cost_medium() {
        let test_input = vec![
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ];
        assert_eq!(find_cost(&grid(test_input)), 1930);
    }
}
