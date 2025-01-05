// Day 15: Warehouse Woes

fn main() -> anyhow::Result<()> {
    let input = rust_advent::read_file_as_string("15")?;
    let (warehouse, moves) = parse_input(&input)?;
    let warehouse = apply_moves(&warehouse, &moves);
    println!("GPS after moves: {}", warehouse.gps());
    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Items {
    Empty,
    Box,
    Wall,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
    row: u8, // row 0 is the top.
    col: u8, // col 0 is the left.
}

impl Position {
    fn move_in_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Position {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Position {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Position {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Warehouse {
    grid: Vec<Vec<Items>>,
    robot: Position,
}

impl TryFrom<&str> for Warehouse {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut grid = Vec::new();
        let mut robot: Option<Position> = None;
        for (row, line) in value.lines().enumerate() {
            let mut output_row = Vec::new();
            for (col, c) in line.chars().enumerate() {
                match c {
                    '.' => output_row.push(Items::Empty),
                    '#' => output_row.push(Items::Wall),
                    'O' => output_row.push(Items::Box),
                    '@' => {
                        robot = Some(Position {
                            row: row as u8,
                            col: col as u8,
                        });
                        output_row.push(Items::Empty);
                    }
                    _ => {
                        return Err(anyhow::anyhow!(
                            "Invalid character: {} at (row: {}, col: {})",
                            c,
                            col,
                            row
                        ))
                    }
                }
            }
            grid.push(output_row);
        }

        Ok(Self {
            grid,
            robot: robot.ok_or(anyhow::anyhow!("No robot found"))?,
        })
    }
}

impl Warehouse {
    fn move_robot(&mut self, direction: Direction) {
        let position_after_first_move = self.robot.move_in_direction(direction);
        let mut curr_position = position_after_first_move;
        loop {
            match self[curr_position] {
                Items::Empty => {
                    // We can move.  Everything between the position after the first move and
                    // the current position is something we can push.
                    self.swap(position_after_first_move, curr_position);
                    self.robot = position_after_first_move;
                    break;
                }
                Items::Wall => {
                    // Robot tried to move either itself or a string of boxes into a wall, denied.
                    break;
                }
                Items::Box => {
                    // A box, keep going.
                    curr_position = curr_position.move_in_direction(direction);
                }
            }
        }
    }

    fn swap(&mut self, position1: Position, position2: Position) {
        let t = self.grid[position1.row as usize][position1.col as usize];
        self.grid[position1.row as usize][position1.col as usize] =
            self.grid[position2.row as usize][position2.col as usize];
        self.grid[position2.row as usize][position2.col as usize] = t;
    }

    fn gps(&self) -> u32 {
        let mut total = 0;
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, item) in row.iter().enumerate() {
                match *item {
                    Items::Box => total += (row_idx as u32) * 100 + (col_idx as u32),
                    _ => (),
                };
            }
        }
        total
    }
}

impl std::ops::Index<Position> for Warehouse {
    type Output = Items;

    fn index(&self, index: Position) -> &Self::Output {
        &self.grid[index.row as usize][index.col as usize]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn apply_moves(warehouse: &Warehouse, moves: &[Direction]) -> Warehouse {
    let mut warehouse = warehouse.clone();
    for &m in moves {
        warehouse.move_robot(m);
    }
    warehouse
}

fn parse_input(input: &str) -> Result<(Warehouse, Vec<Direction>), anyhow::Error> {
    let input_split = input.split("\n\n").collect::<Vec<_>>();
    if input_split.len() != 2 {
        return Err(anyhow::anyhow!(
            "Invalid input; unexpected number of sections"
        ));
    }

    Ok((
        Warehouse::try_from(input_split[0])?,
        parse_moves(input_split[1])?,
    ))
}

fn parse_moves(value: &str) -> Result<Vec<Direction>, anyhow::Error> {
    let mut moves = Vec::new();
    for c in value.chars() {
        if c == '\n' {
            // Newlines were insereted for readability.
            continue;
        }
        moves.push(match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => return Err(anyhow::anyhow!("Invalid move: {}", c)),
        });
    }
    Ok(moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn test_parse_moves() {
        let moves = parse_moves("^>v<").unwrap();
        assert_eq!(
            moves,
            vec![
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left
            ]
        );
    }

    #[test]
    fn move_into_wall() {
        let warehouse = Warehouse::try_from("##@..#").expect("Failed to parse test input");
        assert_eq!(warehouse.robot, Position { row: 0, col: 2 });

        let warehouse_after_move = apply_moves(&warehouse, &[Direction::Left]);
        // Position should not change since we tried to push into a wall.
        assert_eq!(warehouse_after_move, warehouse);
    }

    #[test]
    fn move_into_space() {
        let warehouse = Warehouse::try_from("##@..#").expect("Failed to parse test input");
        assert_eq!(warehouse.robot, Position { row: 0, col: 2 });

        let warehouse_after_move = apply_moves(&warehouse, &[Direction::Right]);
        assert_eq!(warehouse_after_move, Warehouse::try_from("##.@.#").expect("Failed to parse expected output"));
    }

    #[test]
    fn move_box_into_space() {
        let warehouse = Warehouse::try_from("#.O@.#").expect("Failed to parse test input");

        let warehouse_after_move = apply_moves(&warehouse, &[Direction::Left]);
        assert_eq!(warehouse_after_move, Warehouse::try_from("#O@..#").expect("Failed to parse expected output"));
    }

    #[test]
    fn move_two_boxes_into_space() {
        let warehouse = Warehouse::try_from("#.#.OO@.#").expect("Failed to parse test input");

        let warehouse_after_move = apply_moves(&warehouse, &[Direction::Left]);
        assert_eq!(warehouse_after_move, Warehouse::try_from("#.#OO@..#").expect("Failed to parse expected output"));
    }

    #[test]
    fn move_many_boxes_into_space() {
        let warehouse = Warehouse::try_from("#.#@OOOOOO..#.#").expect("Failed to parse test input");

        let warehouse_after_move = apply_moves(&warehouse, &[Direction::Right]);
        assert_eq!(warehouse_after_move, Warehouse::try_from("#.#.@OOOOOO.#.#").expect("Failed to parse expected output"));
    }

    #[test]
    fn move_box_into_wall() {
        let warehouse = Warehouse::try_from("#.#OO@.#").expect("Failed to parse test input");

        let warehouse_after_move = apply_moves(&warehouse, &[Direction::Left]);
        assert_eq!(warehouse_after_move, warehouse);
    }

    #[test]
    fn multiple_moves_small_example() {
        let (warehouse, moves) = parse_input(SMALL_EXAMPLE).expect("Failed to parse input");
        assert_eq!(moves.len(), 15);

        let expected = Warehouse::try_from("########\n#....OO#\n##.....#\n#.....O#\n#.#O@..#\n#...O..#\n#...O..#\n########").expect("Failed to parse expected output");
        assert_eq!(apply_moves(&warehouse, &moves), expected);
    }

    #[test]
    fn small_example_gps() {
        let (warehouse, moves) = parse_input(SMALL_EXAMPLE).expect("Failed to parse input");
        assert_eq!(apply_moves(&warehouse, &moves).gps(), 2028);
    }
}
