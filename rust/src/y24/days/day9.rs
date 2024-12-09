use std::fmt::Formatter;
use crate::*;

pub const DAY9_EXAMPLE: &str =
"2333133121414131402";

pub struct Day9 {
    
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Page {
    Free,
    Used { id: usize },
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Block {
    page: Page,
    start: usize,
    size: usize,
}

#[derive(Debug)]
struct Disk {
    pages: Vec<Page>,
}

impl Display for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for b in self.pages.iter() {
            write!(f, "{}", match b {
                Page::Free => '.'.to_string(),
                Page::Used { id} => id.to_string(),
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
        // println!("{disk}");
        match part {
            Part::One => disk.compact_p1(),
            Part::Two => disk.compact_p2(),
        };
        // println!("{disk}");
        
        disk.checksum()
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY9_EXAMPLE, 1928),
                (self.input(), 6461289671426),
            ],
            test_cases![
                (DAY9_EXAMPLE, 2858),
                (self.input(), 6488291456470),
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
        let mut pages: Vec<Page> = Default::default();

        let mut next_page_free = false;
        let mut id = 0;
        for count_c in input.trim_end().chars() {
            let count = count_c.to_digit(10).unwrap() as usize;
            if next_page_free {
                for _ in 0..count {
                    pages.push(Page::Free);
                }
            } else {
                for _ in 0..count {
                    pages.push(Page::Used {id});
                }
                id += 1;
            }
            next_page_free = !next_page_free;
        }

        Self { pages }
    }

    fn compact_p1(&mut self) {
        let mut left_head = 0;
        let mut right_head = self.pages.len() - 1;
        loop {
            while let Some(Page::Used {..}) = self.pages.get(left_head) {left_head += 1}
            while let Some(Page::Free) = self.pages.get(right_head) {right_head -= 1}
            if left_head >= right_head {break}

            self.pages.swap(left_head, right_head)
        }
    }
    
    fn compact_p2(&mut self) {
        let mut blocks = self.blocks();
        let mut right_head = blocks.len() - 1;
        let mut first_free = 0;
        while {
            while let Some(Block{page: Page::Free, ..}) = blocks.get(right_head) {right_head -= 1}
            right_head > 0
        } {
            let Block {
                page: Page::Used {id},
                start: file_start,
                size: file_size
            } = blocks[right_head]
                else {unreachable!()};
            
            while let Some(Block{page: Page::Used {..}, ..}) = blocks.get(first_free) {first_free += 1};
            let mut left_head = first_free;
            while {
                while let Some(Block{page: Page::Used {..}, ..}) = blocks.get(left_head) {left_head += 1};
                left_head < right_head
            } {
                let Block {
                    page: Page::Free,
                    start: free_start,
                    size: free_size,
                } = blocks[left_head]
                    else {unreachable!()};
                
                if free_size < file_size {left_head += 1;continue}
                
                // copy pages, then update blocks
                let mut free_ptr = free_start .. (free_start+free_size);
                for used_ptr in file_start..(file_start+file_size) {
                    self.pages.swap(free_ptr.next().unwrap(), used_ptr);
                }
                blocks[left_head].page = Page::Used{id};
                blocks[right_head].page = Page::Free;
                
                let excess_free = free_size - file_size;
                if excess_free > 0 {
                    blocks[left_head].size = file_size;
                    // this would've been a linked list but i cba to fight the borrow checker
                    blocks.insert(left_head+1, Block {
                        page: Page::Free,
                        start: free_start + file_size,
                        size: excess_free
                    });
                    right_head += 1;
                }
                if left_head == first_free {
                    first_free += 1;
                }
                break;
            }
            right_head -= 1;
        }
    }
    fn checksum(&self) -> usize {
        self.pages.iter().enumerate()
            .map(|(i, &p)| match p {
                Page::Free => 0,
                Page::Used { id } => i * id,
            })
            .sum()
    }
    
    fn blocks(&self) -> Vec<Block> {
        self.pages.iter()
            .enumerate()
            .rle_by(|(_, &p)| p)
            .map(|((i, p), size)| Block {page: *p, start: i, size})
            .collect_vec()
    }
}
