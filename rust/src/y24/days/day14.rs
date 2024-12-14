use std::cmp::Ordering;
use num::Integer;
use crate::*;

pub struct Day14 {
    
}

#[derive(Debug, Clone)]
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
                    let Some(quadrant) = robot.do_move(time, bounds)
                        else {continue};
                    quadrants[quadrant] += 1;
                    // println!("{robot:?} -> {} in {quadrant}", robot.pos);
                }
                // println!("{}", Self::robot_grid(bounds, &moved_robots));
                
                quadrants.iter().product::<usize>()
            }
            Part::Two => {
                let mut t = 0;
                loop {
                    let mut moved_robots = robots.clone();
                    for robot in &mut moved_robots {
                        robot.do_move(t, bounds);
                    }
                    // pure dumb luck that this worked
                    let grid = Self::robot_grid(bounds, &moved_robots);
                    if grid.elements().all(|&c| c == '.' || c == '1') {
                        println!("\n\nt={t}:\n{grid}");
                        return t;
                    }
                    t += 1;
                }
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
p=9,5 v=-3,-3
"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 12),
                (Self::INPUT, 216027840),
            ],
            test_cases![
                // (Self::EXAMPLES[0], 0),
                // (Self::INPUT, 6876),
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

    fn robot_grid(bounds: Size, moved_robots: &Vec<Robot>) -> Grid<char> {
        let mut grid: Grid<char> = Grid::fill_with(bounds, '.').unwrap();
        for robot in moved_robots {
            grid[robot.pos] = match grid[robot.pos].to_digit(10) {
                None => '1',
                Some(c) => (c + 1).to_string().chars().next().unwrap(),
            };
        }
        grid
    }
}

impl Robot {
    fn do_move(&mut self, steps: usize, bounds: Size) -> Option<usize> {
        let w = bounds.width as isize;
        let h = bounds.height as isize;
        
        self.pos += self.vel * steps;
        
        self.pos.x %= w;
        self.pos.y %= h;
        // modulo of negative
        if self.pos.x < 0 {self.pos.x += w}
        if self.pos.y < 0 {self.pos.y += h}
        
        let x_half = self.pos.x.cmp(&(w/2));
        let y_half = self.pos.y.cmp(&(h/2));
        match (x_half, y_half) {
            (Ordering::Less, Ordering::Less) => Some(0),
            (Ordering::Greater, Ordering::Less) => Some(1),
            (Ordering::Less, Ordering::Greater) => Some(2),
            (Ordering::Greater, Ordering::Greater) => Some(3),
            (_, _) => {None}
        }
    }
}