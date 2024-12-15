// Day 13: Claw Contraption
//
// These are linear equations, so there is only one solution, and we don't need to
// find the best solution, just if there is an integer solution.

const PRIZE_OFFSET: i64 = 10000000000000;

fn main() -> std::io::Result<()> {
    let machines = parse_input(&rust_advent::read_file_as_string("13")?);
    println!(
        "Cost {}",
        machines.iter().filter_map(|m| m.cost()).sum::<i64>()
    );
    println!(
        "Cost with offset {}",
        machines.iter()
        .map(|machine| ClawMachine{
            prize_x: machine.prize_x + PRIZE_OFFSET,
            prize_y: machine.prize_y + PRIZE_OFFSET,
            ..*machine })
        .filter_map(|m| m.cost()).sum::<i64>()
    );

    Ok(())
}

#[derive(Debug)]
struct ClawMachine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

#[derive(Debug, Eq, PartialEq)]
struct ClawSolution {
    n_a: i64,
    n_b: i64,
}

impl ClawMachine {
    fn solve(&self) -> Option<ClawSolution> {
        // Solve using integer math and check the remainder to see if there is an integral solution.
        let n_b_numerator = self.a_x * self.prize_y - self.prize_x * self.a_y;
        let n_b_denominator = self.a_x * self.b_y - self.a_y * self.b_x;
        let n_b = n_b_numerator / n_b_denominator;
        let n_a_numerator = self.prize_x - n_b * self.b_x;
        let n_a_denominator = self.a_x;
        if n_a_numerator % n_a_denominator == 0 && n_b_numerator % n_b_denominator == 0 {
            Some(ClawSolution {
                n_a: n_a_numerator / n_a_denominator,
                n_b: n_b_numerator / n_b_denominator,
            })
        } else {
            None
        }
    }

    fn cost(&self) -> Option<i64> {
        self.solve().map(|solution| 3 * solution.n_a + solution.n_b)
    }
}

fn parse_input(s: &str) -> Vec<ClawMachine> {
    s.split("\n\n")
        .map(|machine| {
            // Each chunk is of the form:
            // Button A: X+11, Y+73
            // Button B: X+95, Y+99
            // Prize: X=6258, Y=10706
            // This is pretty ugly brute force parsing.
            let mut lines = machine
                .lines()
                .map(|line| line.split_once(", ").expect("Missing ,"));
            let (a_x, a_y) = lines.next().expect("Missing Button A line");
            let a_x = a_x
                .trim_start_matches("Button A: X+")
                .parse()
                .expect("Failed to parse Button A X as int");
            let a_y = a_y
                .trim_start_matches("Y+")
                .parse()
                .expect("Failed to parse Button A Y as int");
            let (b_x, b_y) = lines.next().expect("Missing Button B line");
            let b_x = b_x
                .trim_start_matches("Button B: X+")
                .parse()
                .expect("Failed to parse Button B X as int");
            let b_y = b_y
                .trim_start_matches("Y+")
                .parse()
                .expect("Failed to parse Button B Y as int");
            let (prize_x, prize_y) = lines.next().expect("Missing Prize line");
            let prize_x = prize_x
                .trim_start_matches("Prize: X=")
                .parse()
                .expect("Failed to parse Prize X as int");
            let prize_y = prize_y
                .trim_start_matches("Y=")
                .parse()
                .expect("Failed to parse Prize Y as int");
            ClawMachine {
                a_x,
                a_y,
                b_x,
                b_y,
                prize_x,
                prize_y,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_good_example() {
        let machine = ClawMachine {
            a_x: 94,
            a_y: 34,
            b_x: 22,
            b_y: 67,
            prize_x: 8400,
            prize_y: 5400,
        };
        assert_eq!(machine.solve(), Some(ClawSolution { n_a: 80, n_b: 40 }));
    }

    #[test]
    fn test_solve_bad_example() {
        let machine = ClawMachine {
            a_x: 26,
            a_y: 66,
            b_x: 67,
            b_y: 21,
            prize_x: 12748,
            prize_y: 12176,
        };
        assert_eq!(machine.solve(), None);
    }

    #[test]
    fn test_cost_good_example() {
        let machine = ClawMachine {
            a_x: 94,
            a_y: 34,
            b_x: 22,
            b_y: 67,
            prize_x: 8400,
            prize_y: 5400,
        };
        assert_eq!(machine.cost(), Some(280));
    }

    #[test]
    fn test_cost_bad_example() {
        let machine = ClawMachine {
            a_x: 26,
            a_y: 66,
            b_x: 67,
            b_y: 21,
            prize_x: 12748,
            prize_y: 12176,
        };
        assert_eq!(machine.cost(), None);
    }
}
