// Day 7: Bridge repair.

fn main() -> std::io::Result<()> {
    let input = rust_advent::read_file_as_lines("07")?;
    let puzzles = to_number_puzzles(&input);

    println!(
        "Sum of goals of solveable puzzles (without concat): {}",
        puzzles
            .iter()
            .filter(|puzzle| is_solveable(puzzle))
            .map(|puzzle| puzzle.goal)
            .sum::<i64>()
    );
    println!(
        "Sum of goals of solveable puzzles (with concat): {}",
        puzzles
            .iter()
            .filter(|puzzle| is_solveable_with_concat(puzzle))
            .map(|puzzle| puzzle.goal)
            .sum::<i64>()
    );
 
    Ok(())
}

struct NumberPuzzle {
    goal: i64,
    numbers: Vec<i64>,
}

fn to_number_puzzles(input: &[String]) -> Vec<NumberPuzzle> {
    input
        .iter()
        .map(|line| {
            let (goal, nums) = line.split_once(": ").expect("Line did not contain a colon");
            NumberPuzzle {
                goal: goal.parse().expect("Non-number found before ':'"),
                numbers: nums
                    .split(' ')
                    .map(|num| num.parse().expect("Non-number found after ':'"))
                    .collect(),
            }
        })
        .collect()
}

fn is_solveable(puzzle: &NumberPuzzle) -> bool {
    fn is_solveable_inner(goal: i64, current: i64, items: &[i64]) -> bool {
        if current > goal {
            return false;
        }
        if items.is_empty() {
            return goal == current;
        }

        // Using +.
        is_solveable_inner(goal, current + items[0], &items[1..])
        // Using *.
        || is_solveable_inner(goal, current * items[0], &items[1..])
    }

    if puzzle.numbers.is_empty() {
        false
    } else {
        is_solveable_inner(puzzle.goal, puzzle.numbers[0], &puzzle.numbers[1..])
    }
}

fn is_solveable_with_concat(puzzle: &NumberPuzzle) -> bool {
    fn is_solveable_inner(goal: i64, current: i64, items: &[i64]) -> bool {
        if current > goal {
            return false;
        }
        if items.is_empty() {
            return goal == current;
        }

        // Using +.
        is_solveable_inner(goal, current + items[0], &items[1..])
        // Using *.
        || is_solveable_inner(goal, current * items[0], &items[1..])
        // Using concat.
        || is_solveable_inner(goal, concat_numbers(current, items[0]), &items[1..])
    }

    if puzzle.numbers.is_empty() {
        false
    } else {
        is_solveable_inner(puzzle.goal, puzzle.numbers[0], &puzzle.numbers[1..])
    }
}

fn concat_numbers(left: i64, right: i64) -> i64 {
    left * 10_i64.pow(right.ilog10() as u32 + 1) + right
}

