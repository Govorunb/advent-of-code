use crate::*;

aoc_day!(
    day = 11,
    output = usize,
    examples = [
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
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 374),
            (Self::INPUT, 9556712),
        ],
        test_cases![
            (Self::EXAMPLES[0], 82000210),
            (Self::INPUT, 678626199476),
        ]
    ],
    solve = |input, part| {
        let mut grid = SparseGalaxiesGrid::parse(input);
        let expansion = match part {
            Part::One => 1,
            Part::Two => 999999,
        };
        grid.expand(expansion);
        grid.galaxies.iter().enumerate().fold(0, |acc1, (i, gal1)| {
            grid.galaxies.iter().enumerate().skip(i+1).fold(acc1, |acc2, (_, gal2)| {
                acc2 + gal1.cartesian_distance(*gal2)
            })
        })
    }
);


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SparseGalaxiesGrid {
    galaxies: Vec<Vector2>,
    size: Size,
}

impl SparseGalaxiesGrid {
    fn parse(input: &str) -> Self {
        let lines = input.lines();
        let height = lines.clone().count();
        let width = lines.clone().next().unwrap().len();
        let galaxies = lines.enumerate()
            .flat_map(|(y, line)| line.char_indices().map(move |(x, c)| (x, y, c)))
            .filter_map(|(x, y, c)| match c { '#' => Some((x,y).into()), _ => None })
            .collect_vec();

        Self {
            galaxies,
            size: Size { width, height }
        }
    }

    fn expand(&mut self, expansion_factor: isize) {
        let empty_rows = (0..self.size.height as isize)
            .filter(|&y| !self.galaxies.iter().any(|g| g.y == y))
            .collect_vec();
        let empty_columns = (0..self.size.width as isize)
            .filter(|&x| !self.galaxies.iter().any(|g| g.x == x))
            .collect_vec();

        for g in self.galaxies.iter_mut() {
            let empty_rows_before = empty_rows.partition_point(|&y| g.y > y);
            let empty_cols_before = empty_columns.partition_point(|&x| g.x > x);
            *g += Vector2::from((empty_cols_before, empty_rows_before)) * expansion_factor;
        }
    }
}
