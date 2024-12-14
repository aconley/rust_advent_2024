// Day 11: Plutonian pebbles

use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let numbers = rust_advent::read_numbers_with_whitespace("11")?;
    println!(
        "Pebble iterator after 25 steps: {:?}",
        PebbleIterator::new(numbers.clone())
            .nth(24)  // Starts at 0.
            .expect("Failed to get 25th element")
            .len()
    );
    println!(
        "Number of pebbles after 75 steps: {}",
        count_pebbles(&numbers, 75)
    );
 
    Ok(())
}

// For part 1, we actually explicitly construct the list of stones.
struct PebbleIterator {
    numbers: Vec<u64>,
}

impl PebbleIterator {
    fn new(numbers: Vec<u64>) -> Self {
        Self { numbers }
    }
}

impl Iterator for PebbleIterator {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_state = Vec::with_capacity(2 * self.numbers.len());
        for n in self.numbers.iter().cloned() {
            if n == 0 {
                new_state.push(1);
                continue;
            }

            let numlen = base10_len(n);
            if numlen & 1 == 0 {
                // Even.
                let (left, right) = split_number_at(n, numlen / 2);
                new_state.push(left);
                new_state.push(right);
            } else {
                new_state.push(n * 2024);
            }
        }
        self.numbers = new_state;
        Some(self.numbers.clone())
    }
}

fn base10_len(n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    n.ilog10() as usize + 1
}

fn split_number_at(n: u64, at: usize) -> (u64, u64) {
    let divisor = 10_u64.pow(at as u32);
    (n / divisor, n % divisor)
}

// For part 2, constructing the list is impractical, so we
// just use a recursive function with memoization to count
// the pebbles.

fn count_pebbles(numbers: &[u64], n_steps: usize) -> u64 {
    fn count_pebbles_inner(value: u64, step: usize, memo: &mut HashMap<(u64, usize), u64>) -> u64 {
        if step == 0 {
            // Last step, we have 1 pebble.
            return 1;
        }

        // See if we already computed this.
        if let Some(&cached_result) = memo.get(&(value, step)) {
            return cached_result;
        }

        // Recurse to compute.
        let result = if value == 0 {
            count_pebbles_inner(1, step - 1, memo)
        } else if base10_len(value) & 1 == 0 {
            let (left, right) = split_number_at(value, base10_len(value) / 2);
            count_pebbles_inner(left, step - 1, memo) + count_pebbles_inner(right, step - 1, memo)
        } else {
            count_pebbles_inner(value * 2024, step - 1, memo)
        };
        memo.insert((value, step), result);
        result
    }

    let mut memo = HashMap::new();
    numbers.iter().cloned().map(|n| count_pebbles_inner(n, n_steps, &mut memo)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base10_len1() {
        assert_eq!(base10_len(0), 1);
        assert_eq!(base10_len(9), 1);
    }

    #[test]
    fn test_base10_len3() {
        assert_eq!(base10_len(100), 3);
        assert_eq!(base10_len(123), 3);
        assert_eq!(base10_len(999), 3);
    }

    #[test]
    fn test_split_number_at_half() {
        assert_eq!(split_number_at(1234, 2), (12, 34));
    }


    #[test]
    fn test_split_number_at() {
        assert_eq!(split_number_at(12345, 1), (1234, 5));
        assert_eq!(split_number_at(12345, 2), (123, 45));
        assert_eq!(split_number_at(12345, 3), (12, 345));
        assert_eq!(split_number_at(12345, 4), (1, 2345));
    }

    #[test]
    fn test_advance_pebble_iterator() {
        let mut pebble_iterator = PebbleIterator::new(vec![125, 17]);

        assert_eq!(pebble_iterator.next(), Some(
            vec![253000, 1, 7]));
        assert_eq!(pebble_iterator.next(), Some(
            vec![253, 0, 2024, 14168]));
        assert_eq!(pebble_iterator.next(), Some(
            vec![512072, 1, 20, 24, 28676032]));
        assert_eq!(pebble_iterator.next(), Some(
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]));
        assert_eq!(pebble_iterator.next(), Some(
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]));
        assert_eq!(pebble_iterator.next(), Some(
            vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]));
    }

    #[test]
    fn test_count_pebbles_iterator() {
        let mut pebble_iterator = PebbleIterator::new(vec![125, 17]);
        assert_eq!(pebble_iterator.nth(24).unwrap().len(), 55312);
    }

    #[test]
    fn test_count_pebbles() {
        assert_eq!(count_pebbles(&vec![125, 17], 25,), 55312);
    }
}
