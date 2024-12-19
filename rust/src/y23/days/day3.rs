use crate::*;

pub struct Day3;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Digit(usize),
    Symbol,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '0'..='9' => Cell::Digit(c.to_digit(10).unwrap().try_into().unwrap()),
            _ => Cell::Symbol
        }
    }
}

impl Cell {
    fn is_digit(&self) -> bool {
        matches!(self, Cell::Digit(_))
    }
}

impl Day<3> for Day3 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day3.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let (sum, gear_ratio_sum) = self.do_it(input);
        match part {
            Part::One => sum,
            Part::Two => gear_ratio_sum,
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 4361),
                (Self::INPUT, 549908),
            ],
            test_cases![
                (Self::EXAMPLES[0], 467835),
                (Self::INPUT, 81166799),
            ]
        ]
    }
}

impl Day3 {
    fn get_number(grid: &Grid<Cell>, x: usize, y: usize) -> Option<(usize, usize, usize)> {
        grid.row(y)
            // cell at given coords must be inside a number
            .filter(|row| row.get(x).is_some_and(Cell::is_digit))
            .map(|row| {
                // todo: this is a flood fill
                // go left until there's no digit
                let left = (0..x).rev()
                    .find(|i| !row[*i].is_digit())
                    .map(|i| i+1)
                    .unwrap_or(0);
                // go right until there's no digit
                let right = ((x+1)..row.len())
                    .find(|i| !row[*i].is_digit())
                    .unwrap_or(row.len())-1;
                let num: usize = row[left..=right]
                    .iter()
                    .map(|c| {
                        let Cell::Digit(d) = c else { unreachable!() };
                        d
                    }).fold(0, |acc, &d| acc*10 + d);
                (num, left, right)
        })
    }

    fn do_it(&self, input: &str) -> (usize, usize) {
        let grid: Grid<Cell> = input.parse()
            .expect("failed to parse grid");
        let mut touched: FxHashSet<(usize, usize)> = FxHashSet::default();
        let mut sum: usize = 0;
        let mut gear_ratio_sum: usize = 0;
        for pt in grid.coords() {
            let (x,y) = (pt.x as usize, pt.y as usize);
            if !matches!(grid[pt], Cell::Symbol) {continue}
            
            let mut adjacent_to: Vec<usize> = Vec::new();
            for x2 in x - 1..=x + 1 {
                for y2 in y - 1..=y + 1 {
                    if touched.contains(&(x2, y2)) {continue}
                    if let Some((n, n_x1, n_x2)) = Self::get_number(&grid, x2, y2) {
                        adjacent_to.push(n);
                        sum += n;
                        for n_x in n_x1..=n_x2 {
                            touched.insert((n_x, y2));
                        }
                    } else {continue}
                }
            }
            if adjacent_to.len() == 2 {
                gear_ratio_sum += adjacent_to[0] * adjacent_to[1];
            }
        }
        
        (sum, gear_ratio_sum)
    }
}
