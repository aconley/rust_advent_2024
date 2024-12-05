// Day 4

fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_ascii_grid("04")?;
    println!("Number of XMASs: {}", count_in_grid(&inputs, b"XMAS"));
    println!("Number of X-MASs: {}", count_x_in_grid(&inputs, b"MAS"));
    Ok(())
}

// Returns true if the character at the specified location is in bounds and equal
// to the expected value.
fn is_expected(input: &[Vec<u8>], row_idx: i32, col_idx: i32, expected: u8) -> bool {
    return row_idx >= 0
        && row_idx < (input.len() as i32)
        && col_idx >= 0
        && col_idx < (input[row_idx as usize].len() as i32)
        && input[row_idx as usize][col_idx as usize] == expected;
}

// Counts the number of occurrences of the specified string in the grid.
fn count_in_grid(inputs: &[Vec<u8>], word: &[u8]) -> u32 {
    if inputs.is_empty() || inputs[0].is_empty() {
        return 0;
    }

    let mut count = 0;
    for (row_idx, row) in inputs.iter().enumerate() {
        for (col_idx, val) in row.iter().enumerate() {
            if *val != word[0] {
                continue;
            }

            for (direction_row, direction_col) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                if word.iter().enumerate().skip(1).all(|(word_idx, word_val)| {
                    is_expected(
                        inputs,
                        (row_idx as i32) + (word_idx as i32) * direction_row,
                        (col_idx as i32) + (word_idx as i32) * direction_col,
                        *word_val,
                    )
                }) {
                    count += 1
                }
            }
        }
    }
    count
}

fn count_x_in_grid(inputs: &[Vec<u8>], word: &[u8]) -> u32 {
    assert!(word.len() & 1 == 1, "Word must have odd length");
    let half_len = word.len() / 2;
    let central_char = word[half_len];

    let mut count = 0;
    for (row_idx, row) in inputs.iter().enumerate() {
        for (col_idx, val) in row.iter().cloned().enumerate() {
            if val != central_char {
                continue;
            }

            // TODO: clean up to take advantage of short circuiting and reduce
            // duplication.
            let has_nwse_diag = 
                word.iter().cloned().enumerate().all(|(word_idx, word_val)| {
                    is_expected(inputs,
                        row_idx as i32 - half_len as i32 + word_idx as i32,
                        col_idx as i32 - half_len as i32 + word_idx as i32,
                       word_val
                    )
                });
            let has_senw_diag =
                word.iter().cloned().enumerate().all(|(word_idx, word_val)| {
                    is_expected(inputs,
                        row_idx as i32 + half_len as i32 - word_idx as i32,
                        col_idx as i32 + half_len as i32 - word_idx as i32,
                        word_val
                    )
                });
            let has_swne_diag =
                word.iter().cloned().enumerate().all(|(word_idx, word_val)| {
                    is_expected(inputs,
                        row_idx as i32 + half_len as i32 - word_idx as i32,
                        col_idx as i32 - half_len as i32 + word_idx as i32,
                        word_val
                    )
                });
            let has_nesw_diag =
                word.iter().cloned().enumerate().all(|(word_idx, word_val)| {
                    is_expected(inputs,
                        row_idx as i32 - half_len as i32 + word_idx as i32,
                        col_idx as i32 + half_len as i32 - word_idx as i32,
                        word_val
                    )
                });
            if (has_nwse_diag || has_senw_diag) && (has_swne_diag || has_nesw_diag) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_xmas_grid() {
        let test_grid = vec![
            b"MMMSXXMASM",
            b"MSAMXMSMSA",
            b"AMXSXMAAMM",
            b"MSAMASMSMX",
            b"XMASAMXAMM",
            b"XXAMMXXAMA",
            b"SMSMSASXSS",
            b"SAXAMASAAA",
            b"MAMMMXMMMM",
            b"MXMXAXMASX",
        ]
        .iter()
        .map(|row| row.to_vec())
        .collect::<Vec<Vec<u8>>>();

        assert_eq!(count_in_grid(&test_grid, b"XMAS"), 18);
    }

    #[test]
    fn example_xmas_grid_x() {
        let test_grid = vec![
            b"MMMSXXMASM",
            b"MSAMXMSMSA",
            b"AMXSXMAAMM",
            b"MSAMASMSMX",
            b"XMASAMXAMM",
            b"XXAMMXXAMA",
            b"SMSMSASXSS",
            b"SAXAMASAAA",
            b"MAMMMXMMMM",
            b"MXMXAXMASX",
        ]
        .iter()
        .map(|row| row.to_vec())
        .collect::<Vec<Vec<u8>>>();

        assert_eq!(count_x_in_grid(&test_grid, b"MAS"), 9);
    }
}