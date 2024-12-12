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
        let combos = std::iter::repeat_n(0..100, ingredients.len())
            .multi_cartesian_product()
            .filter(|p| p.iter().sum::<isize>() == 100);
        // a less bruteforcey solution would have been:
        // start equal (e.g. 25/25/25/25); step in some direction, measure change (higher/lower)
        // basically gradient descent (or ascent in this case)
        // let start = std::iter::repeat_n(100/ingredients.len(), ingredients.len()).collect_vec();
        
        match part {
            Part::One => {
                combos
                    .map(|counts| Self::score(&ingredients, &counts))
                    .max().unwrap()
            },
            Part::Two => {
                let combos = combos
                    .filter(|counts|
                        ingredients.iter().zip(counts)
                            .map(|(i,&c)| i.calories * c as usize)
                            .sum::<usize>() == 500
                    );
                // let x = combos
                //     .map(|counts| (Self::score(&ingredients, &counts), counts))
                //     .max_by(|a, b| a.0.cmp(&b.0))
                //     .unwrap();
                // println!("{:?}", x.1);
                // x.0
                
                combos
                    .map(|counts| Self::score(&ingredients, &counts))
                    .max().unwrap()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY15_EXAMPLE, 62842880),
                (self.input(), 18965440), // solved this one manually in desmos https://www.desmos.com/calculator/ehfhvcx2ka
            ],
            test_cases![
                (DAY15_EXAMPLE, 57600000),
                (self.input(), 15862900),
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
    
    fn score(ingredients: &[Ingredient], counts: &[isize]) -> usize {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        for (i, &c) in ingredients.iter().zip(counts.iter()) {
            capacity += c * i.capacity;
            durability += c * i.durability;
            flavor += c * i.flavor;
            texture += c * i.texture;
        }
        (capacity.max(0)
            * durability.max(0)
            * flavor.max(0)
            * texture.max(0)
        ) as usize
    }
}