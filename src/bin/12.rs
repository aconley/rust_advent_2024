// Day 12: Garden Groups

use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let input = rust_advent::read_ascii_grid("12")?;
    println!("Compute fence cost {}", find_cost(&input));
    println!("Compute discount fence cost {}", find_discount_cost(&input));
    Ok(())
}

const DIRECTIONS: [(i16, i16); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    fn discount_cost(&self) -> u32 {
        let edges = self.get_edges();
        let n_edges = edges
            .iter()
            .map(|edge_loop| edge_loop.len() as u32)
            .sum::<u32>();
        self.cells.len() as u32 * n_edges
    }

    fn exterior_segments(&self) -> HashSet<Segment> {
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

    // Each element is a connected path of edges.
    //
    // There may be multiple paths if the region has holes.
    fn get_edges(&self) -> Vec<Vec<Edge>> {
        let exterior = {
            let mut exterior = self.exterior_segments().into_iter().collect::<Vec<_>>();
            exterior.sort();
            exterior
        };
        let mut edge_loops = Vec::new();
        let mut seen = HashSet::<Segment>::new();

        for segment in &exterior {
            if seen.contains(&segment) {
                // Already visited.
                continue;
            }
            seen.insert(segment.clone());

            // Start a new loop.  We are guaranteed to be at a corner
            // and be able to head up.
            debug_assert!(
                segment.orientation == Orientation2D::Vertical,
                "Unexpected segment orientation: {:?}",
                segment
            );
            let start_point = segment.lower_left;
            let mut current_directed_segment = DirectedSegment {
                start: segment.lower_left,
                direction: Direction::Up,
            };
            let mut edge_loop = Vec::new();
            let mut current_edge = Edge {
                lower_left: segment.lower_left,
                orientation: segment.orientation,
                length: 1,
            };

            // Continue moving around the outside until we reach the start point.
            while current_directed_segment.end() != start_point {
                // We start by trying to turn right.  Doing so rather than first
                // trying to go straight is important to avoid handling 'mobius-strip'
                // cases, as shown in the test_find_discount_cost_mobius_strip test.
                let mut next_directed_segment =
                    current_directed_segment.extend_in_current_direction().turn_right();
                let initial_direction = next_directed_segment.direction;
                // The direction we just came from.
                let reverse_direction = current_directed_segment.direction.opposite();
                while next_directed_segment.direction == reverse_direction
                    || !exterior.contains(&next_directed_segment.to_segment())
                {
                    next_directed_segment = next_directed_segment.turn_right();
                    if next_directed_segment.direction == initial_direction {
                        // We rotated all the way around and didn't find a new edge.
                        panic!("No next edge found from {:?}", current_directed_segment);
                    }
                }

                if next_directed_segment.direction == current_directed_segment.direction {
                    // Continuing in the same direction.  Extend the current edge.
                    current_edge.length += 1;
                    current_edge.lower_left = match next_directed_segment.direction {
                        Direction::Up => current_edge.lower_left,
                        Direction::Right => current_edge.lower_left,
                        Direction::Down => Point {
                            row: current_edge.lower_left.row - 1,
                            col: current_edge.lower_left.col,
                        },
                        Direction::Left => Point {
                            row: current_edge.lower_left.row,
                            col: current_edge.lower_left.col - 1,
                        },
                    };
                } else {
                    // We hit a corner.  Finish the current edge.
                    edge_loop.push(current_edge);

                    // Start a new edge in the new direction.
                    let next_segment = next_directed_segment.to_segment();
                    current_edge = Edge {
                        lower_left: next_segment.lower_left,
                        orientation: next_segment.orientation,
                        length: 1,
                    };
                }
                current_directed_segment = next_directed_segment;
                seen.insert(current_directed_segment.to_segment());
            }
            // We've completed the loop.  Flush the last edge.
            edge_loop.push(current_edge);
            edge_loops.push(edge_loop);
        }
        edge_loops
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

fn find_discount_cost(grid: &[Vec<u8>]) -> u32 {
    find_regions(grid)
        .iter()
        .map(|region| region.discount_cost())
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Orientation2D {
    Vertical,
    Horizontal,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Segment {
    lower_left: Point,
    orientation: Orientation2D,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    // Turn 90 degrees clockwise.
    fn next(&self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct DirectedSegment {
    start: Point,
    direction: Direction,
}

impl DirectedSegment {
    // Return the next segment in the current direction.
    fn extend_in_current_direction(&self) -> Self {
        DirectedSegment {
            start: match self.direction {
                Direction::Up => Point {
                    row: self.start.row + 1,
                    col: self.start.col,
                },
                Direction::Right => Point {
                    row: self.start.row,
                    col: self.start.col + 1,
                },
                Direction::Down => Point {
                    row: self.start.row - 1,
                    col: self.start.col,
                },
                Direction::Left => Point {
                    row: self.start.row,
                    col: self.start.col - 1,
                },
            },
            direction: self.direction,
        }
    }

    fn turn_right(&self) -> Self {
        DirectedSegment {
            start: self.start,
            direction: self.direction.next(),
        }
    }

    fn to_segment(&self) -> Segment {
        Segment {
            lower_left: match self.direction {
                Direction::Up => self.start,
                Direction::Right => self.start,
                Direction::Down => Point {
                    row: self.start.row - 1,
                    col: self.start.col,
                },
                Direction::Left => Point {
                    row: self.start.row,
                    col: self.start.col - 1,
                },
            },
            orientation: match self.direction {
                Direction::Up => Orientation2D::Vertical,
                Direction::Right => Orientation2D::Horizontal,
                Direction::Down => Orientation2D::Vertical,
                Direction::Left => Orientation2D::Horizontal,
            },
        }
    }

    fn end(&self) -> Point {
        match self.direction {
            Direction::Up => Point {
                row: self.start.row + 1,
                col: self.start.col,
            },
            Direction::Right => Point {
                row: self.start.row,
                col: self.start.col + 1,
            },
            Direction::Down => Point {
                row: self.start.row - 1,
                col: self.start.col,
            },
            Direction::Left => Point {
                row: self.start.row,
                col: self.start.col - 1,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Edge {
    lower_left: Point,
    orientation: Orientation2D,
    length: u16,
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
    fn test_exterior_segments_single_cell() {
        let region = Region {
            cells: HashSet::from([Point { row: 1, col: 1 }]),
        };
        assert_eq!(
            region.exterior_segments(),
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
    fn test_exterior_segments_l_shape() {
        let region = Region {
            cells: HashSet::from([
                Point { row: 0, col: 0 },
                Point { row: 0, col: 1 },
                Point { row: 1, col: 0 },
            ]),
        };
        assert_eq!(
            region.exterior_segments(),
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
    fn test_find_edges_single_square() {
        let region = Region {
            cells: HashSet::from([Point { row: 1, col: 1 }]),
        };
        let edges: Vec<HashSet<Edge>> = region
            .get_edges()
            .iter()
            .map(|edge_loop| {
                edge_loop
                    .iter()
                    .map(|edge| edge.clone())
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        let expected = vec![HashSet::from([
            Edge {
                lower_left: Point { row: 1, col: 1 },
                orientation: Orientation2D::Vertical,
                length: 1,
            },
            Edge {
                lower_left: Point { row: 2, col: 1 },
                orientation: Orientation2D::Horizontal,
                length: 1,
            },
            Edge {
                lower_left: Point { row: 1, col: 2 },
                orientation: Orientation2D::Vertical,
                length: 1,
            },
            Edge {
                lower_left: Point { row: 1, col: 1 },
                orientation: Orientation2D::Horizontal,
                length: 1,
            },
        ])];
        assert_eq!(edges, expected);
    }

    #[test]
    fn test_find_edges_l_shape() {
        let region = Region {
            cells: HashSet::from([
                Point { row: 0, col: 0 },
                Point { row: 1, col: 0 },
                Point { row: 0, col: 1 },
            ]),
        };
        let edges: Vec<HashSet<Edge>> = region
            .get_edges()
            .iter()
            .map(|edge_loop| {
                edge_loop
                    .iter()
                    .map(|edge| edge.clone())
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        let expected = vec![HashSet::from([
            Edge {
                lower_left: Point { row: 0, col: 0 },
                orientation: Orientation2D::Vertical,
                length: 2,
            },
            Edge {
                lower_left: Point { row: 2, col: 0 },
                orientation: Orientation2D::Horizontal,
                length: 1,
            },
            Edge {
                lower_left: Point { row: 1, col: 1 },
                orientation: Orientation2D::Vertical,
                length: 1,
            },
            Edge {
                lower_left: Point { row: 1, col: 1 },
                orientation: Orientation2D::Horizontal,
                length: 1,
            },
            Edge {
                lower_left: Point { row: 0, col: 2 },
                orientation: Orientation2D::Vertical,
                length: 1,
            },
            Edge {
                lower_left: Point { row: 0, col: 0 },
                orientation: Orientation2D::Horizontal,
                length: 2,
            },
        ])];
        assert_eq!(edges, expected);
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

    #[test]
    fn test_find_discount_cost_mini() {
        let test_input = vec!["AA", "BB"];
        assert_eq!(find_discount_cost(&grid(test_input)), 16);
    }

    #[test]
    fn test_find_discount_cost_small() {
        let test_input = vec!["AAAA", "BBCD", "BBCC", "EEEC"];
        assert_eq!(find_discount_cost(&grid(test_input)), 80);
    }

    #[test]
    fn test_find_discount_cost_e() {
        let test_input = vec!["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"];
        assert_eq!(find_discount_cost(&grid(test_input)), 236);
    }

    #[test]
    fn test_find_discount_cost_mobius_strip() {
        // The fences around the Bs form two loops.
        let test_input = vec!["AAAAAA", "AAABBA", "AAABBA", "ABBAAA", "ABBAAA", "AAAAAA"];
        assert_eq!(find_discount_cost(&grid(test_input)), 368);
    }

    #[test]
    fn test_find_discount_cost_medium() {
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
        assert_eq!(find_discount_cost(&grid(test_input)), 1206);
    }
}
