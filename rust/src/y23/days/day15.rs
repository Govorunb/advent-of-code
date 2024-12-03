use crate::test_cases;
use crate::common::*;

pub const DAY15_INPUT: &str = include_str!("../Input/day15.txt");
pub const DAY15_EXAMPLE_HASH: &str =
"HASH";
pub const DAY15_EXAMPLE: &str = 
"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

pub struct Day15 {
    
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Hasher {
    state: u8
}
impl Hasher {
    fn write_byte(&mut self, c: u8) {
        self.state = self.state.wrapping_add(c).wrapping_mul(17);
    }

    fn write_bytes(&mut self, input: &[u8]) {
        for &c in input {
            self.write_byte(c);
        }
    }

    fn finish(&self) -> u8 {
        self.state
    }

    fn hash(input: &str) -> u8 {
        let mut h = Hasher {state: 0};
        h.write_bytes(input.as_bytes());
        h.finish()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    boxes: Vec<Vec<Lens>>,
}

impl Map {
    fn power(&self) -> usize {
        self.boxes.iter()
            .enumerate()
            .map(|(i, r#box)| {
                let box_sum: usize = r#box.iter()
                    .enumerate()
                    .map(|(j,l)| (j+1) * l.focal_length)
                    .sum();
                (i+1) * box_sum
            })
            .sum()
    }
}

impl Default for Map {
    fn default() -> Self {
        Self { boxes: vec![Vec::new(); 256] }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Lens {
    name: String,
    focal_length: usize,
}

impl Day<15> for Day15 {
    type Output = usize;
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let init_sequence = input
            .split(',');
        match part {
            Part::One => {
                init_sequence
                    .map(Hasher::hash)
                    .fold(0, |acc, h| acc + h as usize)
            },
            Part::Two => {
                let mut map = Map::default();

                for step in init_sequence {
                    let last = step.chars().last().unwrap();

                    let label = &step[..step.len()-match last {
                        '-' => 1,
                        '0'..='9' => 2,
                        _ => unreachable!()
                    }];
                    
                    let h = Hasher::hash(label);
                    let r#box = &mut map.boxes[h as usize];
                    match last {
                        '-' => { r#box.retain(|lens| lens.name != label); },
                        '0'..='9' => {
                            let focal_length = last.to_digit(10).unwrap() as usize;
                            if let Some(existing_lens) = r#box.iter_mut().find(|l| { l.name == label }) {
                                existing_lens.focal_length = focal_length
                            } else {
                                r#box.push(Lens {
                                    name: label.to_owned(),
                                    focal_length
                                })
                            }
                        },
                        _ => unreachable!()
                    }
                }

                map.power()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY15_EXAMPLE_HASH, 52),
                (DAY15_EXAMPLE, 1320),
                (DAY15_INPUT, 505427),
            ],
            test_cases![
                (DAY15_EXAMPLE, 145),
                (DAY15_INPUT, 243747),
            ]
        ]
    }
}

impl Default for Day15 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day15 {
    pub fn new() -> Self {
        Self {
        }
    }
}