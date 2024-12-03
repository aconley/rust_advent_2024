/// Day 2.
fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_number_grid("02")?;
    println!(
        "Number monotonic within bound of differences: {}",
        count_monotonic_bound(&inputs, 3)
    );
    Ok(())
}

/// Given a list of lists of integers.
///
/// Counts the number of lists that are monotonic with absolute differences between
/// successive elements less than the specified bands.
fn count_monotonic_bound(input: &[Vec<i32>], bound: i32) -> usize {
    input
        .iter()
        .filter(|values| is_monotonic_within_bound(*values, bound))
        .count()
}

// Returns true if a vector is monotonic with all differences <= the specified bound.
fn is_monotonic_within_bound(values: &[i32], bound: i32) -> bool {
    if values.len() < 2 {
        return true;
    }

    if values[0] == values[1] {
        false
    } else if values[0] > values[1] {
        // Decreasing.
        let mut prev_val = values[0];
        for val in values[1..].iter().cloned() {
            if (val >= prev_val) || (prev_val - val > bound) {
                return false;
            }
            prev_val = val;
        }
        true
    } else {
        // Increasing.
        let mut prev_val = values[0];
        for val in values[1..].iter().cloned() {
            if (val <= prev_val) || (val - prev_val > bound) {
                return false;
            }
            prev_val = val;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_array_is_monotonic() {
        assert!(is_monotonic_within_bound(&vec![], 3));
    }

    #[test]
    fn array_with_identical_values_is_not_monotonic() {
        assert!(!is_monotonic_within_bound(&vec![8, 6, 4, 4, 1], 3));
    }

    #[test]
    fn increasing_array_within_bounds() {
        assert!(is_monotonic_within_bound(&vec![1, 3, 6, 7, 9], 3));
    }

    #[test]
    fn decreasing_array_within_bounds() {
        assert!(is_monotonic_within_bound(&vec![7, 6, 4, 2, 1], 3));
    }

    #[test]
    fn overly_large_increase() {
        assert!(!is_monotonic_within_bound(&vec![1, 2, 7, 8, 9], 3));
    }

    #[test]
    fn overly_large_decrease() {
        assert!(!is_monotonic_within_bound(&vec![9, 7, 6, 2, 1], 3));
    }

    #[test]
    fn non_monotonic() {
        assert!(!is_monotonic_within_bound(&vec![9, 7, 6, 2, 1], 3));
        assert!(!is_monotonic_within_bound(&vec![1, 3, 2, 4, 5], 3));
    }

    #[test]
    fn monotonic_bound_example() {
        let input = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(count_monotonic_bound(&input, 3), 2);
    }
}
