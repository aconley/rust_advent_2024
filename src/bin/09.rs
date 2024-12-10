// Day 9: Disk Fragmenter
fn main() -> std::io::Result<()> {
    let input = rust_advent::read_file_as_string("09")?;

    println!("Checksum after defrag: {}", defrag_and_checksum(&input));
    println!(
        "Checksum after defrag files: {}",
        defrag_and_checksum_files(&input)
    );
    Ok(())
}

fn defrag_and_checksum(input: &str) -> u64 {
    let mut disk = Disk::new_from_string(input);
    disk.defrag();
    disk.checksum()
}

fn defrag_and_checksum_files(input: &str) -> u64 {
    let mut disk = DiskList::new_from_string(input);
    disk.defrag();
    disk.checksum()
}

struct Disk {
    // The value u16::MAX is used to represent an empty slot.
    blocks: Vec<u16>,
}

impl Disk {
    fn new_from_string(input: &str) -> Self {
        let elems: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiskList {
    files: Vec<File>,
    free_space: Vec<FreeSpace>,
}

impl DiskList {
    fn new_from_string(input: &str) -> Self {
        let elems: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

        let mut files = Vec::with_capacity(elems.len() / 2 + 1);
        let mut free_space = Vec::with_capacity(elems.len() / 2);
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
                    free_space.push(FreeSpace {
                        start: idx,
                        size: elem as usize,
                    });
                    state = ParseState::Data;
                }
                ParseState::Data => {
                    files.push(File {
                        id,
                        start: idx,
                        size: elem as usize,
                    });
                    id += 1;
                    state = ParseState::Empty;
                }
            }
            idx += elem as usize;
        }

        Self { files, free_space }
    }

    fn defrag(&mut self) {
        self.files.iter_mut().rev().for_each(|fl| {
            if let Some(free) = self
                .free_space
                .iter_mut()
                .find(|free| free.start < fl.start && free.size >= fl.size)
            {
                // We found a free chunk to move the file into.
                // Do so, reducing the free space chunk.
                fl.start = free.start;
                free.start += fl.size;
                free.size -= fl.size;

                // This implementation is not quite right -- we need to create a new free space chunk
                // in the space we moved out of, and possibly merge with adjacent free space chunks.
                // However, that doesn't affect the answer:
                //  1) The checksum doesn't care about the free space.
                //  2) Because we are iterating backwards over the files, no other file can move
                //    into the space we just moved out of.
            }
        });

        self.files.sort_by_key(|fl| fl.start);
        self.free_space.retain(|free| free.size > 0);
    }

    fn checksum(&self) -> u64 {
        self.files.iter().map(|fl| fl.checksum()).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct File {
    id: u16,
    start: usize,
    // A size of 0 means the file is deactivated.
    size: usize,
}

impl File {
    fn checksum(&self) -> u64 {
        if self.size == 0 {
            0
        } else {
            // There is surely a closed form for this.
            (self.start..(self.start + self.size))
                .into_iter()
                .map(|idx| idx as u64 * self.id as u64)
                .sum()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FreeSpace {
    start: usize,
    size: usize,
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

    #[test]
    fn test_defrag_files() {
        let mut disk = DiskList::new_from_string("2333133121414131402");
        disk.defrag();
        assert_eq!(disk.checksum(), 2858);
    }
}
