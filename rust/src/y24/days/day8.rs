use std::collections::HashMap;
use std::str::FromStr;
use itertools::Either;
use crate::*;

aoc_day!(
    day = 8,
    output = usize,
    examples = [
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
............"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 14),
            (Self::INPUT, 323),
        ],
        test_cases![
            (Self::EXAMPLES[0], 34),
            (Self::INPUT, 1077),
        ]
    ],
    solve = |input, part| {
        let grid: Grid<Symbol> = Grid::from_str(input).unwrap();
        let antennas = Self::find_antennas(&grid);
        let antinodes = Self::place_antinodes(&antennas, grid.bounds(), part);
        
        // println!("{antinodes:?}");
        
        antinodes.len()
    }
);


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

impl Day8 {
    fn find_antennas(grid: &Grid<Symbol>) -> HashMap<char, Vec<Antenna>> {
        grid.cells()
            .filter_map(|(pt, &c)| {
                if let Symbol::Antenna(freq) = c {
                    Some(Antenna { pos: pt, freq })
                } else {None}
            })
            .into_group_map_by(|a| a.freq)
    }
    
    fn place_antinodes(antennas_map: &HashMap<char, Vec<Antenna>>, bounds: Rect, part: Part) -> FxHashSet<Vector2> {
        let mut result = FxHashSet::default();
        for (a1, a2) in antennas_map.iter()
            .flat_map(|(_freq, antennas)| antennas.iter().cartesian_product(antennas.iter()))
        {
            if a1.pos == a2.pos {continue}
            
            let distance = a2.pos - a1.pos;
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