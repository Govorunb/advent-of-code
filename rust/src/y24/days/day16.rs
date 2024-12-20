use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap};
use std::str::FromStr;
use pathfinding::prelude::{astar_bag, AstarSolution};
use crate::*;

aoc_day!(
    day = 16,
    output = usize,
    examples = [
"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 7036),
            (Self::EXAMPLES[1], 11048),
            (Self::INPUT, 108504),
        ],
        test_cases![
            (Self::EXAMPLES[0], 45),
            (Self::EXAMPLES[1], 64),
            (Self::INPUT, 538),
        ]
    ],
    solve = |input, part| {
        let grid: Grid<char> = Grid::from_str(input).unwrap();
        let (paths, cost) = Self::search(&grid);
        match part {
            Part::One => cost,
            Part::Two => {
                let mut set = FxHashSet::default();
                for path in paths {
                    set.extend(path.iter().map(|node| node.0));
                }
                set.len()
            }
        }
    }
);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Node(Vector2, Direction);

impl Day16 {
    const TURN_COST: usize = 1000;
    
    fn search(grid: &Grid<char>) -> (AstarSolution<Node>, usize) {
        let start_pos = grid.find(&'S').unwrap();
        let goal_pos = grid.find(&'E').unwrap();
        
        let start = Node(start_pos, Direction::East);
        // println!("{grid}");
        
        let heuristic = |node: &Node| -> usize {
            let &Node(pos, dir) = node;
            if pos == goal_pos {return 0}
            
            let to_goal = goal_pos - pos;
            let turns = if let Ok(dir_to_goal) = Direction::try_from(to_goal) {
                match Turn::from_corner(dir, dir_to_goal) {
                    Turn::None => 0,
                    Turn::Left | Turn::Right => 1,
                    Turn::Opposite => 2,
                }
            } else {
                // at least 1 turn (need to walk both x and y)
                // then, at most 1 more turn if facing away from goal
                let vec: Vector2 = dir.into();
                if to_goal.x.signum() == vec.x.signum() || to_goal.y.signum() == vec.y.signum() { 
                    1
                } else {
                    2
                }
            };
            let dist_cost = (to_goal.x.abs() + to_goal.y.abs()) as usize;
            turns * Self::TURN_COST + dist_cost
        };
        let cost = |a: Node, b: Node| -> usize {
            let Node(pt_a, dir_a) = a;
            let Node(pt_b, dir_b) = b;
            let dist = pt_a.manhattan_distance(pt_b);
            debug_assert!(dist <= 1);
            
            let turns = match Turn::from_corner(dir_a, dir_b) {
                Turn::None => 0,
                Turn::Left | Turn::Right => 1,
                Turn::Opposite => 2,
            };
            
            turns * Self::TURN_COST + dist
        };
        
        let neighbours = |node: &Node| {
            Direction::all_clockwise()
                .into_iter()
                .filter_map(move |dir| {
                    if dir == -node.1 {return None}
                    
                    let next_pos: Vector2 = node.0 + dir;
                    grid.get(&next_pos)
                        .filter(|&&c| c != '#')
                        .map(|_| {
                            let next = Node(next_pos, dir);
                            let c = cost(*node, next);
                            (next, c)
                        })
                }).collect_vec()
        };
        
        astar_bag(&start, neighbours, heuristic, |n| n.0 == goal_pos).unwrap()
    }
}
