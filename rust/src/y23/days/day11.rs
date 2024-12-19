use crate::*;

pub struct Day11;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SparseGalaxiesGrid {
    galaxies: Vec<Point>,
    size: Size,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl SparseGalaxiesGrid {
    fn parse(input: &str) -> Self {
        let lines = input.lines();
        let height = lines.clone().count();
        let width = lines.clone().next().unwrap().len();
        let galaxies = lines.enumerate()
            .flat_map(|(y, line)| line.char_indices().map(move |(x, c)| (x, y, c)))
            .filter_map(|(x, y, c)| match c { '#' => Some(Point{x, y}), _ => None })
            .collect_vec();

        Self {
            galaxies,
            size: Size { width, height }
        }
    }

    fn expand(&mut self, expansion_factor: usize) {
        let empty_rows = (0..self.size.height)
            .filter(|&y| !self.galaxies.iter().any(|g| g.y == y))
            .collect_vec();
        let empty_columns = (0..self.size.width)
            .filter(|&x| !self.galaxies.iter().any(|g| g.x == x))
            .collect_vec();

        for g in self.galaxies.iter_mut() {
            let empty_rows_before = empty_rows.partition_point(|&y| g.y > y);
            g.y += expansion_factor * empty_rows_before;

            let empty_cols_before = empty_columns.partition_point(|&x| g.x > x);
            g.x += expansion_factor * empty_cols_before;
        }
        // let expand_row = self.galaxies.iter().map(|g| empty_rows.iter().filter(|&&y| g.y > y).count()).collect_vec();
        // let expand_col = self.galaxies.iter().map(|g| empty_columns.iter().filter(|&&x| g.x > x).count()).collect_vec();

        // for (i, times) in expand_row.iter().enumerate() {
        //     self.galaxies[i].y += expansion_factor * times;
        // }

        // for (i, times) in expand_col.iter().enumerate() {
        //     self.galaxies[i].x += expansion_factor * times;
        // }
    }

    fn dist(&self, a: &Point, b: &Point) -> usize {
        let dx = b.x as i64 - a.x as i64;
        let dy = b.y as i64 - a.y as i64;
        (dx.abs() + dy.abs()) as usize
    }
}

impl Day<11> for Day11 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day11.txt");

    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let mut grid = SparseGalaxiesGrid::parse(input);
        let expansion = match part {
            Part::One => 1,
            Part::Two => 999999,
        };
        grid.expand(expansion);
        grid.galaxies.iter().enumerate().fold(0, |acc1, (i, gal1)| {
            grid.galaxies.iter().enumerate().skip(i+1).fold(acc1, |acc2, (_, gal2)| {
                let dist = grid.dist(gal1, gal2);
                acc2 + dist
            })
        })
    }
    const EXAMPLES: &'static [&'static str] = &[
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 374),
                (Self::INPUT, 9556712),
            ],
            test_cases![
                (Self::EXAMPLES[0], 82000210),
                (Self::INPUT, 678626199476),
            ]
        ]
    }
}


