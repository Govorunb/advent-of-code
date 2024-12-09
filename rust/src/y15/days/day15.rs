// replace all 15 with the day number
use crate::*;

pub const DAY15_EXAMPLE: &str =
"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";

pub struct Day15 {
    
}

struct Ingredient {
    name: String,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: usize,
}

impl Day<15> for Day15 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day15.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let regex = Regex::new(r#"(?<name>\w+): capacity (?<capacity>-?\d+), durability (?<durability>-?\d+), flavor (?<flavor>-?\d+), texture (?<texture>-?\d+), calories (?<calories>-?\d+)"#).unwrap();
        let ingredients = regex.captures_iter(input)
            .map(|c| {
                Ingredient {
                    name: c.parse("name"),
                    capacity: c.parse("capacity"),
                    durability: c.parse("durability"),
                    flavor: c.parse("flavor"),
                    texture: c.parse("texture"),
                    calories: c.parse("calories"),
                }
            }).collect_vec();
        match part {
            Part::One => {
                ingredients.len()
            },
            Part::Two => {
                0
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY15_EXAMPLE, 62842880),
                // (self.input(), 0),
            ],
            test_cases![
                // (DAY15_EXAMPLE, 0),
                // (self.input(), 0),
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