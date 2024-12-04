use regex::Regex;

// Day 3.
fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_file_as_string("03")?;
    println!("Sum of multiplies: {}", sum_of_multiplies(&inputs));
    println!("Conditional sum of multiplies: {}", conditional_sum_of_multiplies(&inputs));
    Ok(())
}

fn sum_of_multiplies(inputs: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to create regex");
    re.captures_iter(inputs)
        .map(|c| c.extract())
        .map(|(_, [v1, v2])| v1.parse::<i64>().unwrap() * v2.parse::<i64>().unwrap())
        .sum()
}

fn conditional_sum_of_multiplies(inputs: &str) -> i64 {
    let mut enabled = true;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("Failed to create regex");
    let mut sum = 0;
    for capture in re.captures_iter(inputs) {
        match &capture[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    sum += capture[1].parse::<i64>().unwrap() * capture[2].parse::<i64>().unwrap();
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_has_sum_zero() {
        assert_eq!(sum_of_multiplies(""), 0);
    }

    #[test]
    fn no_match_has_sum_zero() {
        assert_eq!(sum_of_multiplies("abcmul(b,4)cdmul(3,)mul(2, 3)"), 0);
    }

    #[test]
    fn single_sum() {
        assert_eq!(sum_of_multiplies("mul(11,3)"), 33);
    }

    #[test]
    fn sum_of_multiplies_example() {
        assert_eq!(
            sum_of_multiplies(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ),
            161
        );
    }

    #[test]
    fn single_sum_enabled() {
        assert_eq!(conditional_sum_of_multiplies("don't()do()mul(11,3)"), 33);
    }

    #[test]
    fn single_sum_disabled() {
        assert_eq!(conditional_sum_of_multiplies("don't()mul(11,3)"), 0);
    }

    #[test]
    fn conditional_sum_of_multiplies_example() {
        assert_eq!(
            conditional_sum_of_multiplies(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        );
    }
}
