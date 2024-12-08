use std::collections::HashMap;
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
    
    fn find_antennas(grid: &Grid<Symbol>) -> HashMap<char, Vec<Antenna>> {
        grid.cells()
            .filter_map(|(pt, &c)| {
                if let Symbol::Antenna(freq) = c {
                    Some(Antenna { pos: pt, freq })
                } else {None}
            })
            .into_group_map_by(|a| a.freq)
    }
    
    fn place_antinodes(grid: &Grid<Symbol>, antennas_map: &HashMap<char, Vec<Antenna>>, part: Part) -> FxHashSet<Vector2> {
        let mut result = FxHashSet::default();
        for (a1, a2) in antennas_map.iter()
            .flat_map(|(_freq, antennas)| antennas.iter().cartesian_product(antennas.iter()))
        {
            if a1.pos == a2.pos {continue}
            
            let distance = a2.pos - a1.pos;
            let bounds = grid.bounds();
            let rays_data = [(a2.pos, distance), (a1.pos, -distance)];
            
            let rays = rays_data.map(|(start, step)| 
                start.ray(step).take_while(|p| bounds.contains(p))
            );
            
            for ray in rays {
                match part {
                    // one pair of antinodes, located `distance` away from each antenna
                    Part::One => result.extend(ray.skip(1).take(1)),
                    // at all points along the line that are multiples of `distance` away (even 0)
                    Part::Two => result.extend(ray),
                }
            }
        }
        
        result
    }
}