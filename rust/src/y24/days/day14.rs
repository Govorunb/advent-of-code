use std::cmp::Ordering;
use num::Integer;
use crate::*;

pub struct Day14 {
    
}

#[derive(Debug, Clone, PartialEq)]
struct Robot {
    pos: Vector2,
    vel: Vector2,
}

impl Day<14> for Day14 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day14.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let regex = Regex::new(r#"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)"#).unwrap();
        let robots = regex.captures_iter(input)
            .map(|c|{
                Robot {
                    pos: c.vec2("px", "py"),
                    vel: c.vec2("vx", "vy"),
                }
            }).collect_vec();
        let bounds: Size = match input.lines().count() {
            ..20 => (11, 7).into(), // example
            _ => (101, 103).into(), // input
        };
        match part {
            Part::One => {
                let time = 100;
                let mut quadrants = [0usize;4];
                
                let mut moved_robots = robots.clone(); 
                for robot in &mut moved_robots {
                    let Some(quadrant) = Robot::quadrant(robot.do_move(time, bounds), bounds)
                        else {continue};
                    quadrants[quadrant] += 1;
                    // println!("{robot:?} -> {} in {quadrant}", robot.pos);
                }
                
                quadrants.iter().product()
            }
            Part::Two => {
                // clever solution - idea by x8uurg
                // each robot's X coordinate necessarily has a cycle of at most W (width), likewise for Y
                // then, the following must hold:
                // target % bounds.width = target_x
                // target % bounds.height = target_y
                // instead of searching W * H, we can search W and H separately (W + H)
                // then, repeatedly add e.g. H to target_y and compare it (mod W) to target_x
                // the total computation cost should be at most 2*max(W,H)
                let mut x_cycle = (0,0); // t, max_xs
                let mut y_cycle = (0,0); // t, max_ys
                for t in 0..bounds.width.max(bounds.height) {
                    let mut moved_robots = robots.clone();
                    let mut xs = vec![0usize; bounds.width];
                    let mut ys = vec![0usize; bounds.height];
                    for robot in &mut moved_robots {
                        robot.pos = robot.do_move(t, bounds);
                        xs[robot.pos.x as usize] += 1;
                        ys[robot.pos.y as usize] += 1;
                    }
                    let max_xs = *xs.iter().max().unwrap();
                    let max_ys = *ys.iter().max().unwrap();
                    if x_cycle.1 < max_xs {x_cycle = (t, max_xs)}
                    if y_cycle.1 < max_ys {y_cycle = (t, max_ys)}
                }
                
                let target_x = x_cycle.0;
                let target_y = y_cycle.0;
                
                // println!("{target_x}:{target_y}");
                
                let target = (0..bounds.width)
                    .map(|n| target_y + bounds.height * n)
                    .find(|t| t % bounds.width == target_x)
                    .unwrap();
                if cfg!(debug_assertions) {
                    let moved_robots = robots.iter()
                        .map(|r| {
                            Robot {
                                pos: r.do_move(target, bounds),
                                vel: r.vel,
                            }
                        })
                        .collect_vec();
                    println!("{}", Self::grid(bounds, &moved_robots));
                }
                target
            },
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 12),
                (Self::INPUT, 216027840),
            ],
            test_cases![
                (Self::INPUT, 6876),
            ]
        ]
    }
}

impl Default for Day14 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day14 {
    pub fn new() -> Self {
        Self {
        }
    }

    fn grid(bounds: Size, moved_robots: &Vec<Robot>) -> Grid<char> {
        let mut grid = Grid::from_origin(bounds).unwrap();
        for robot in moved_robots {
            grid[robot.pos] += 1;
        }
        grid.map_clone(|count| match count {
            0 => '.',
            &c => char::from_digit(c as u32, 10).unwrap(),
        })
    }
}

impl Robot {
    fn do_move(&self, steps: usize, bounds: Size) -> Vector2 {
        (self.pos + self.vel * steps).wrap(bounds)
    }
    
    fn quadrant(pos: Vector2, bounds: Size) -> Option<usize> {
        let w = bounds.width as isize;
        let h = bounds.height as isize;
        
        let x_half = pos.x.cmp(&(w / 2));
        let y_half = pos.y.cmp(&(h / 2));
        match (x_half, y_half) {
            (Ordering::Less, Ordering::Less) => Some(0),
            (Ordering::Greater, Ordering::Less) => Some(1),
            (Ordering::Less, Ordering::Greater) => Some(2),
            (Ordering::Greater, Ordering::Greater) => Some(3),
            (_, _) => {None}
        }
    }
}