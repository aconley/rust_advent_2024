// Day 14:  Restroom Redoubt

use anyhow::anyhow;
use itertools::Itertools;
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
    let input = rust_advent::read_file_as_string("14")?;
    let robot_grid = RobotGrid::new_from_str(&input, 101, 103)?;
    println!(
        "Quad score after 100 timesteps: {}",
        advance_and_multiply_quads(&robot_grid, 100)
    );

    // Hope that the minimum quad score shows a christmas tree, but verify.
    let timestep_of_minimum_score = find_minimum_score(&robot_grid);
    println!(
        "After {} timesteps, the score is minimized with grid:\n{}",
        timestep_of_minimum_score, robot_grid.advance_by(timestep_of_minimum_score)
    );
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug, Eq, PartialEq)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl TryFrom<&str> for Robot {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((_, pos_x, pos_y, _, v_x, v_y)) = value.split(&['=', ',', ' ']).next_tuple() {
            Ok(Self {
                position: Point {
                    x: pos_x.parse::<i16>()?,
                    y: pos_y.parse::<i16>()?,
                },
                velocity: Point {
                    x: v_x.parse::<i16>()?,
                    y: v_y.parse::<i16>()?,
                },
            })
        } else {
            Err(anyhow!("failed to parse robot"))
        }
    }
}

struct RobotGrid {
    robots: Vec<Robot>,
    width: i16,
    height: i16,
}

impl RobotGrid {
    fn new_from_str(value: &str, width: i16, height: i16) -> Result<Self, anyhow::Error> {
        Ok(Self {
            robots: value
                .lines()
                .map(|line| Robot::try_from(line))
                .collect::<Result<Vec<_>, _>>()?,
            width,
            height,
        })
    }

    fn advance_by(&self, timesteps: i16) -> Self {
        Self {
            robots: self
                .robots
                .iter()
                .map(|robot| Robot {
                    position: Point {
                        x: (robot.position.x as i64 + robot.velocity.x as i64 * timesteps as i64)
                            .rem_euclid(self.width as i64) as i16,
                        y: (robot.position.y as i64 + robot.velocity.y as i64 * timesteps as i64)
                            .rem_euclid(self.height as i64) as i16,
                    },
                    ..*robot
                })
                .collect(),
            width: self.width,
            height: self.height,
        }
    }

    // Count the number of robots in each quadrant.
    //
    // Robots on the axis between quadrants are not counted.
    fn count_quads(&self) -> [u16; 4] {
        let mut quad_count = [0; 4];
        let midpoint_x = self.width / 2;
        let midpoint_y = self.height / 2;
        for robot in &self.robots {
            if robot.position.x < midpoint_x && robot.position.y < midpoint_y {
                quad_count[0] += 1;
            } else if robot.position.x > midpoint_x && robot.position.y < midpoint_y {
                quad_count[1] += 1;
            } else if robot.position.x < midpoint_x && robot.position.y > midpoint_y {
                quad_count[2] += 1;
            } else if robot.position.x > midpoint_x && robot.position.y > midpoint_y {
                quad_count[3] += 1;
            }
        }
        quad_count
    }
}

impl std::fmt::Display for RobotGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = vec![vec![' '; self.width as usize]; self.height as usize];
        for robot in &self.robots {
            grid[robot.position.y as usize][robot.position.x as usize] = '#';
        }
        for row in grid {
            writeln!(f, "{}", row.iter().join(""))?;
        }
        Ok(())
    }
}

fn advance_and_multiply_quads(grid: &RobotGrid, timesteps: i16) -> u32 {
    grid.advance_by(timesteps)
        .count_quads()
        .iter()
        .map(|&x| x as u32)
        .product()
}

// Returns the timestep at which the score is minimized.
fn find_minimum_score(grid: &RobotGrid) -> i16 {
    // The grid must repeat in X every width steps, and in Y every height steps,
    // so we don't need to consider more than that (really, their LCM).
    let max_timesteps = grid.width * grid.height;
    (1..max_timesteps)
        .into_par_iter()
        .map(|timestep| (timestep, advance_and_multiply_quads(grid, timestep)))
        .min_by_key(|(_, score)| *score)
        .map(|(timestep, _)| timestep)
        .expect("Should have taken at least one step")
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::assert_ok_eq;

    const TEST_GRID: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_parse_robot() {
        assert_ok_eq!(
            Robot::try_from("p=89,76 v=-46,-5"),
            Robot {
                position: Point { x: 89, y: 76 },
                velocity: Point { x: -46, y: -5 },
            }
        );
    }

    #[test]
    fn test_count_quads() {
        let grid = RobotGrid::new_from_str(TEST_GRID, 11, 7).expect("failed to parse grid");
        assert_eq!(grid.count_quads(), [4, 0, 2, 2]);
    }

    #[test]
    fn test_advance_and_multiply_quads() {
        let grid = RobotGrid::new_from_str(TEST_GRID, 11, 7).expect("failed to parse grid");
        assert_eq!(advance_and_multiply_quads(grid, 100), 12);
    }
}
