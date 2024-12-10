// Day 9: Disk Fragmenter
fn main() -> std::io::Result<()> {
    let input = rust_advent::read_file_as_string("09")?;
    let mut disk = Disk::new_from_string(&input);

    // Part 1: defrag and checksum.
    disk.defrag();
    println!("Checksum after defrag: {}", disk.checksum());

    Ok(())
}

struct Disk {
    // The value u16::MAX is used to represent an empty slot.
    blocks: Vec<u16>,
}

impl Disk {
    fn new_from_string(input: &str) -> Self {
        let elems: Vec<u32> = input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        let n_blocks = elems.iter().sum::<u32>() as usize;
        let mut blocks = vec![u16::MAX; n_blocks];

        enum ParseState {
            Empty,
            Data,
        }
        let mut state = ParseState::Data;
        let mut id = 0;
        let mut idx = 0;
        for elem in elems {
            match state {
                ParseState::Empty => {
                    // Block is already initialized to the empty value, so just skip ahead.
                    state = ParseState::Data;
                }
                ParseState::Data => {
                    blocks[idx..(idx + elem as usize)].fill(id as u16);
                    id += 1;
                    state = ParseState::Empty;
                }
            }
            idx += elem as usize;
        }
        Self { blocks }
    }

    fn defrag(&mut self) {
        let mut l = 0;
        let mut r = self.blocks.len() - 1;
        while l < r {
            if self.blocks[r] == u16::MAX {
                r -= 1;
            } else if self.blocks[l] != u16::MAX {
                l += 1;
            } else {
                self.blocks[l] = self.blocks[r];
                self.blocks[r] = u16::MAX;
                l += 1;
                r -= 1;
            }
        }
    }

    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .cloned()
            .filter(|b| *b != u16::MAX)
            .enumerate()
            .map(|(i, b)| i * (b as usize))
            .sum::<usize>() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_string() {
        let disk = Disk::new_from_string("2333133121414131402");
        assert_eq!(
            disk.blocks,
            vec![
                0,
                0,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                1,
                1,
                1,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                2,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                3,
                3,
                3,
                u16::MAX,
                4,
                4,
                u16::MAX,
                5,
                5,
                5,
                5,
                u16::MAX,
                6,
                6,
                6,
                6,
                u16::MAX,
                7,
                7,
                7,
                u16::MAX,
                8,
                8,
                8,
                8,
                9,
                9
            ]
        );
    }

    #[test]
    fn test_defrag() {
        let mut disk = Disk::new_from_string("2333133121414131402");
        disk.defrag();

        let mut expected = vec![
            0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
        ];
        expected.extend(vec![u16::MAX; 14]);
        assert_eq!(disk.blocks, expected);
    }

    #[test]
    fn test_checksum() {
        let mut disk = Disk::new_from_string("2333133121414131402");
        disk.defrag();
        assert_eq!(disk.checksum(), 1928);
    }
}
