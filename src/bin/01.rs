/// Day 1.
fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_int_pairs("01")?;
    println!("Sum of differences: {}", sum_of_differences(&inputs.0, &inputs.1));
    println!("Similarity score: {}", similarity_score(&inputs.0, &inputs.1));
    Ok(())
}


/// Given two lists of integers, pair the smallest integer from the
/// first list with the smallest integer from the second list, the
/// second smallest integer from the first list with the second
/// smallest integer from the second list, and so on.
/// 
/// Return the sum of the absolute differences between the two lists.
fn sum_of_differences(v1: &[i32], v2: &[i32]) -> i32 {
    let mut v1 = v1.to_vec();
    let mut v2 = v2.to_vec();
    v1.sort();
    v2.sort();
    v1.into_iter().zip(v2).map(|(a, b)| (b - a).abs()).sum()
}

// Given two lists of integers, return the sum of each value
// in the first list times how many times that value appears in the
// second list.
fn similarity_score(v1: &[i32], v2: &[i32]) -> i32 {
    let mut counts_in_v2 = std::collections::HashMap::new();
    for val in v2 {
        *counts_in_v2.entry(*val).or_insert(0) += 1;
    }
    v1.iter().map(|val: &i32| val * counts_in_v2.get(val).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_differences_simple() {
        assert_eq!(sum_of_differences(&vec![1, 4, 2], &vec![5, 3, 7]), 8);
    }

    #[test]
    fn test_sum_of_differences_single_values() {
        assert_eq!(sum_of_differences(&vec![1, 1, 1], &vec![2, 2, 2]), 3);
    }

    #[test]
    fn test_sum_of_differences_identical() {
        let v = vec![3, 2, 7, 0, 11];
        assert_eq!(sum_of_differences(&v, &v), 0);
    }
 
    #[test]
    fn test_sum_of_differences_empty() {
        assert_eq!(sum_of_differences(&vec![], &vec![]), 0);
    }

    #[test]
    fn test_sum_of_differences_example() {
        assert_eq!(sum_of_differences(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]), 11);
    }

    #[test]
    fn test_sum_of_differences_symmetric() {
        let v1 = vec![3, 4, 2, 1, 3, 3];
        let v2 = vec![4, 3,5, 3, 9 ,3];
        assert_eq!(sum_of_differences(&v1, &v2), sum_of_differences(&v2, &v1));
    }

    #[test]
    fn test_similarity_empty() {
        assert_eq!(similarity_score(&vec![], &vec![]), 0);
    }

    #[test]
    fn test_similarity_non_overlapping() {
        assert_eq!(similarity_score(&vec![1, 2, 3, 4], &vec![5, 6, 7, 8]), 0);
    }

    #[test]
    fn test_similarity_identical() {
        let v = vec![1, 2, 3];
        // Each element only appears once.
        assert_eq!(similarity_score(&v, &v), v.iter().sum());
    }

    #[test]
    fn test_similarity_example() {
        assert_eq!(similarity_score(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]), 31);
    }


}
