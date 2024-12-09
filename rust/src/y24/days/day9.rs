use std::fmt::Formatter;
use crate::*;

pub const DAY9_EXAMPLE: &str =
"2333133121414131402";

pub struct Day9 {
    
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
    Free,
    Used(usize), // id
}

struct SizeBlock {
    block: Block,
    size: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Disk {
    blocks: Vec<Block>
}

impl Display for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for b in self.blocks.iter() {
            write!(f, "{}", match b {
                Block::Free => '.'.to_string(),
                Block::Used(usize) => usize.to_string(),
            })?;
        }
        Ok(())
    }
}

impl Day<9> for Day9 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day9.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let mut disk = Disk::parse(input);
        println!("{disk}");
        
        disk.compact(part);
        println!("{disk}");
        
        disk.checksum()
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY9_EXAMPLE, 1928),
                // (self.input(), 6461289671426),
            ],
            test_cases![
                (DAY9_EXAMPLE, 2858),
                // (self.input(), 0),
            ]
        ]
    }
}

impl Default for Day9 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day9 {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Disk {
    fn parse(input: &str) -> Self {
        let mut blocks: Vec<Block> = Default::default();

        let mut block_free = false;
        let mut i = 0;
        for count_c in input.trim_end().chars() {
            let count = count_c.to_digit(10).unwrap() as usize;
            if block_free {
                for _ in 0..count {
                    blocks.push(Block::Free);
                }
            } else {
                for _ in 0..count {
                    blocks.push(Block::Used(i));
                }
                i += 1;
            }
            block_free = !block_free;
        }

        Self {blocks}
    }

    fn compact(&mut self, part: Part) {
        let mut left_head = 0;
        let mut right_head = self.blocks.len() - 1;
        match part {
            Part::One => {
                loop {
                    while let Some(Block::Used(_)) = self.blocks.get(left_head) {left_head += 1}
                    while let Some(Block::Free) = self.blocks.get(right_head) {right_head -= 1}
                    if left_head >= right_head {break}
        
                    self.blocks.swap(left_head, right_head)
                }
            },
            Part::Two => {
                loop {
                    left_head = 0;
                    while let Some(Block::Free) = self.blocks.get(right_head) {right_head -= 1}
                    let id = match self.blocks[right_head] {
                        Block::Used(id) => id,
                        Block::Free => unreachable!(),
                    };
                    // println!("found file at {right_head} - id {id}");
                    let next_free = self.blocks.iter().enumerate()
                        .take(right_head)
                        .rfind(|(_i, &b)| match b {
                            Block::Free => true,
                            Block::Used(id_other) => id_other != id
                        })
                        .unwrap_or((usize::MAX, &Block::Free)).0;
                    if next_free == usize::MAX {
                        // println!("no more files?");
                        break;
                    }
                    let file_size = right_head - next_free;
                    let file_block = (next_free+1)..=right_head;
                    // println!("looking at file {file_block:?} (size {file_size})");
                    
                    // find free block (left head)
                    let mut free_block = None;
                    while free_block.is_none() {
                        // while let Some(Block::Used(_)) = self.blocks.get(left_head) {left_head += 1}
                        left_head = self.blocks.iter().enumerate()
                            .skip(left_head+1)
                            .find(|(_i, b)| matches!(b, Block::Free))
                            .unwrap_or((right_head, &Block::Free)).0;
                        if left_head >= right_head {
                            // println!("no free blocks {left_head}>={right_head}");
                            break;
                        } else {
                            // println!("found free blocks");
                            let next_used = self.blocks.iter().enumerate()
                                .skip(left_head)
                                .find(|(_i, b)| matches!(b, Block::Used(_)))
                                .unwrap().0;
                            let free_size = next_used - left_head;
                            let free_block_ = left_head..next_used;
                            // println!("free block: {free_block_:?}");
                            if free_size >= file_size {
                                // println!("block has good size! {free_size}<={file_size}");
                                free_block = Some(free_block_);
                            }
                        }
                    }
                    if let Some(range) = free_block {
                        // copy block
                        let mut free_ptr = range.clone();
                        for used_ptr in file_block {
                            self.blocks.swap(free_ptr.next().unwrap(), used_ptr);
                        }
                    } else {
                        // println!("did not find suitable block for file {right_head} :(");
                        right_head -= file_size;
                    }
                }
            }
        }
    }

    fn checksum(&self) -> usize {
        self.blocks.iter().enumerate()
            .map(|(i, b)| i * match b {
                Block::Free => 0,
                Block::Used(id) => *id,
            })
            .sum()
    }
}