use std::str::FromStr;
use itertools::Either;
use crate::*;

pub const DAY8_EXAMPLE: &str =
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

pub struct Day8 {
    
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Symbol {
    Empty,
    Antinode,
    Antenna(char),
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        match c {
            '.' => Symbol::Empty,
            '#' => Symbol::Antinode,
            _ => Symbol::Antenna(c)
        }
    }
}

struct Antenna {
    pos: Vector2,
    freq: char,
}

impl Day<8> for Day8 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day8.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let grid: Grid<Symbol> = Grid::from_str(input).unwrap();
        let antennas = Self::find_antennas(&grid);
        let antinodes = Self::place_antinodes(&grid, &antennas, part);
        
        // println!("{antinodes:?}");
        
        antinodes.len()
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY8_EXAMPLE, 14),
                (self.input(), 323),
            ],
            test_cases![
                (DAY8_EXAMPLE, 34),
                (self.input(), 1077),
            ]
        ]
    }
}

impl Default for Day8 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day8 {
    pub fn new() -> Self {
        Self {
        }
    }
    
    fn find_antennas(grid: &Grid<Symbol>) -> Vec<Antenna> {
        grid.cells()
            .filter_map(|(pt, &c)| {
                if let Symbol::Antenna(freq) = c {
                    Some(Antenna { pos: pt, freq })
                } else {None}
            })
            .collect_vec()
    }
    
    fn place_antinodes(grid: &Grid<Symbol>, antennas: &[Antenna], part: Part) -> FxHashSet<Vector2> {
        let mut result = FxHashSet::default();
        for (a1, a2) in antennas.iter().cartesian_product(antennas.iter()) {
            if a1.freq != a2.freq {continue}
            if a1.pos == a2.pos {continue}
            
            let distance = a2.pos - a1.pos;
            
            let bounds = grid.bounds();
            let rays_thing = [(a2.pos, distance), (a1.pos, -distance)];
            let rays = rays_thing.iter()
                .map(|(pt, step)| pt.ray(*step).take_while(|x| bounds.contains(x)))
                .map(|ray| {
                    match part {
                        // part one - one pair of antinodes, located `distance` away from each antenna
                        Part::One => Either::Left(ray.skip(1).take(1)),
                        // part two - at all points along the line that are multiples of `distance` away (even 0)
                        Part::Two => Either::Right(ray),
                    }
                });
            
            for ray in rays {
                result.extend(ray);
            }
        }
        
        result
    }
}