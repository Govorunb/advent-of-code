use std::cmp::Ordering;
use std::fmt::Formatter;
use pathfinding::prelude::astar;
use crate::*;

aoc_day!(
    day = 18,
    output = String,
    examples = [
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
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], "22".to_string()),
            (Self::INPUT, "404".to_string()),
        ],
        test_cases![
            (Self::EXAMPLES[0], "(6,1)".to_string()), // index 20
            (Self::INPUT, "(27,60)".to_string()), // index 2873
        ]
    ],
    solve = |input, part| {
        let bytes = input.lines()
            .map(|line| {
                let (x,y) = line.split_once(',').unwrap();
                Vector2 {
                    x: x.parse::<isize>().unwrap(),
                    y: y.parse::<isize>().unwrap()
                }
            }).collect_vec();
        
        let is_example = input.lines().nth(100).is_none();
        let side = if is_example {7} else {71};
        let size = Size {width: side, height: side};
        
        let mut grid: Grid<char> = Grid::fill_with(size, '.').unwrap();
        let p1_fall_amt = if is_example {12} else {1024};
        for byte in &bytes[..p1_fall_amt] {
            grid[byte] = '#';
        }
        match part {
            Part::One => {
                let (_path, cost) = Self::search(&grid).unwrap();
                // for p in _path {
                //     grid[&p] = 'o';
                // }
                // println!("{grid}");
                
                cost.to_string()
            },
            Part::Two => {
                // binary search is a teeny bit faster (>10x)
                // let (path, _) = Self::search(&grid).unwrap();
                // let mut path_pts: FxHashSet<Vector2> = FxHashSet::from_iter(path);
                // for b in bytes.iter().skip(p1_fall_amt) {
                //     grid[b] = '#';
                //     if !path_pts.contains(b) {continue}
                //     
                //     let Some((path, _)) = Self::search(&grid)
                //         else {return b.to_string()};
                //     path_pts.clear();
                //     path_pts.extend(path);
                // }
                // unreachable!()
                let mut low = p1_fall_amt;
                let mut high = bytes.len()-1;
                while high - low > 1 {
                    let mid = (low + high) / 2;
                    for b in &bytes[low..=mid] {
                        grid[b] = '#';
                    }
                    for b in &bytes[(mid+1)..=high] {
                        grid[b] = '.';
                    }
                    match Self::search(&grid) {
                        Some(_) => low = mid,
                        None => high = mid
                    }
                }
                bytes[high].to_string()
            }
        }
    }
);

impl Day18 {
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
