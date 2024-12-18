use std::fmt::Formatter;
use pathfinding::prelude::astar;
use crate::*;

pub struct Day18 {
    
}

impl Day<18> for Day18 {
    type Output = String;
    const INPUT: &'static str = include_str!("../Input/day18.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let bytes = input.lines()
            .map(|line| {
                let (x,y) = line.split_once(',').unwrap();
                Vector2 {
                    x: x.parse::<isize>().unwrap(),
                    y: y.parse::<isize>().unwrap()
                }
            }).collect_vec();
        
        let is_example = input.lines().count() < 100;
        let side = if is_example {7} else {71};
        let size = Size {width: side, height: side};
        
        let mut grid: Grid<char> = Grid::fill_with(size, '.').unwrap();
        let p1_fall_amt = if is_example {12} else {1024};
        for byte in &bytes[..p1_fall_amt] {
            grid[byte] = '#';
        }
        let (path, cost) = Self::search(&grid).unwrap();
        match part {
            Part::One => {
                // for p in path {
                //     grid[&p] = 'o';
                // }
                // println!("{grid}");
                
                cost.to_string()
            },
            Part::Two => {
                let mut path_pts: FxHashSet<Vector2> = FxHashSet::from_iter(path);
                for b in bytes.iter().skip(p1_fall_amt) {
                    grid[b] = '#';
                    if !path_pts.contains(b) {continue}
                    
                    let Some((path, _)) = Self::search(&grid)
                        else {return b.to_string()};
                    path_pts.clear();
                    path_pts.extend(path);
                }
                unreachable!()
            }
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], "22".to_string()),
                (Self::INPUT, "404".to_string()),
            ],
            test_cases![
                (Self::EXAMPLES[0], "(6,1)".to_string()), // index 20
                (Self::INPUT, "(27,60)".to_string()), // index 2873
            ]
        ]
    }
}

impl Default for Day18 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day18 {
    pub fn new() -> Self {
        Self {
        }
    }

    fn search(grid: &Grid<char>) -> Option<(Vec<Vector2>, usize)> {
        let start = grid.bounds().bottom_right();
        let goal = grid.base();
        let neighbours = |pt: &Vector2| {
            pt.adjacent()
                .filter_map(|p|
                    grid.get(&p)
                        .filter(|&&c| c != '#')
                        .map(|_| (p, 1))
                )
        };
        let heuristic = |pt: &Vector2| pt.cartesian_distance(goal);

        astar(&start, neighbours, heuristic, |&p| p == goal)
    }
}