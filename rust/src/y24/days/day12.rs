use std::str::FromStr;
use crate::*;

pub const DAY12_EXAMPLE: &str =
"AAAA
BBCD
BBCC
EEEC
";

pub const DAY12_EXAMPLE2: &str =
"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

pub const DAY12_EXAMPLE3: &str = 
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

pub const DAY12_EXAMPLE4: &str = 
"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

pub const DAY12_EXAMPLE5: &str = 
"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
pub struct Day12 {
    
}
#[derive(Debug, Clone)]
struct Region {
    plant: char,
    points: Vec<Vector2>,
    fences: FxHashSet<(Vector2, Direction)>,
}

#[derive(Debug, Clone)]
struct Garden {
    grid: Grid<char>,
    regions: Vec<Region>,
}

impl Day<12> for Day12 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day12.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let mut garden = Garden::new(input);
        garden.find_regions();
        match part {
            Part::One => {
                // for region in garden.regions {
                //     println!("{:?}", region);
                // }

                garden.regions.iter()
                    .map(|r| r.points.len() * r.fences.len())
                    .sum()
            },
            Part::Two => {
                garden.regions.iter()
                    .map(|r| r.points.len() * r.sides())
                    .sum()
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY12_EXAMPLE, 140),
                (DAY12_EXAMPLE2, 772),
                (DAY12_EXAMPLE3, 1930),
                (self.input(), 1461752),
            ],
            test_cases![
                (DAY12_EXAMPLE, 80),
                (DAY12_EXAMPLE2, 436),
                (DAY12_EXAMPLE3, 1206),
                (DAY12_EXAMPLE4, 236),
                (DAY12_EXAMPLE5, 368),
                (self.input(), 904114),
            ]
        ]
    }
}

impl Default for Day12 {fn default() -> Self {Self::new()}}
impl Day12 {pub fn new() -> Self {Self {}}}


impl Garden {
    fn new(input: &str) -> Self {
        let grid = Grid::from_str(input).unwrap();
        Self {
            grid,
            regions: vec![],
        }
    }
    
    fn find_regions(&mut self) {
        // classic flood fill
        let mut visited: FxHashSet<Vector2> = FxHashSet::default();
        
        for (pt, &plant) in self.grid.cells() {
            if visited.contains(&pt) {continue}

            let mut white: Vec<Vector2> = vec![pt];
            let mut black: Vec<Vector2> = vec![];
            let mut fences = FxHashSet::default();

            while let Some(tile) = white.pop() {
                if !black.contains(&tile) {
                    black.push(tile);
                }
                for adj in tile.adjacent() {
                    if !black.contains(&adj) && !white.contains(&adj) {
                        if self.grid.get(&adj).is_some_and(|&adj_plant| adj_plant == plant) {
                            white.push(adj);
                        } else {
                            fences.insert((tile, Direction::try_from(adj - tile).unwrap()));
                        }
                    }
                }
            }
            let points = black;
            visited.extend(points.clone());
            self.regions.push(Region {
                plant,
                points,
                fences,
            })
        } 
    }
}

impl Region {
    fn sides(&self) -> usize {
        let mut total = 0;
        let mut visited: FxHashSet<(Vector2, Direction)> = FxHashSet::default();
        for fence in &self.fences {
            if visited.contains(fence) {continue}
            visited.insert(*fence);
            let (pos, direction) = *fence;
            
            let mut side_length = 0;
            for dir in direction.sides() {
                let mut curr = pos;
                while self.fences.contains(&(curr, direction)) {
                    visited.insert((curr, direction));
                    // println!("buh {:?} -> {:?} -> {:?}", (pos, direction), dir, (curr, direction));
                    curr += dir;
                    side_length += 1;
                }
                // println!("{:?} -> {:?} = {:?}", (pos, direction), dir, side_length);
            }
            if side_length > 0 {
                total += 1;
            }
        }
        
        // println!("{self:?} has {total} sides");

        total
    }
}