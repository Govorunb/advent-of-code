use std::str::FromStr;
use crate::*;

aoc_day!(
    day = 12,
    output = usize,
    examples = [
"AAAA
BBCD
BBCC
EEEC",
"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 140),
            (Self::EXAMPLES[1], 772),
            (Self::EXAMPLES[2], 1930),
            (Self::INPUT, 1461752),
        ],
        test_cases![
            (Self::EXAMPLES[0], 80),
            (Self::EXAMPLES[1], 436),
            (Self::EXAMPLES[2], 1206),
            (Self::EXAMPLES[3], 236),
            (Self::EXAMPLES[4], 368),
            (Self::INPUT, 904114),
        ]
    ],
    solve = |input, part| {
        let mut garden = Garden::new(input);
        garden.find_regions();
        match part {
            Part::One => {
                // for region in garden.regions {
                //     println!("{:?}", region);
                // }

                garden.regions.iter()
                    .map(|r| r.points.len() * r.fences.len())
                    .sum()
            },
            Part::Two => {
                garden.regions.iter()
                    .map(|r| r.points.len() * r.sides())
                    .sum()
            }
        }
    }
);

#[derive(Debug, Clone)]
struct Region {
    plant: char,
    points: Vec<Vector2>,
    fences: FxHashSet<(Vector2, Direction)>,
}

#[derive(Debug, Clone)]
struct Garden {
    grid: Grid<char>,
    regions: Vec<Region>,
}

impl Garden {
    fn new(input: &str) -> Self {
        let grid = Grid::from_str(input).unwrap();
        Self {
            grid,
            regions: vec![],
        }
    }
    
    fn find_regions(&mut self) {
        // classic flood fill
        let mut visited: FxHashSet<Vector2> = FxHashSet::default();
        
        for (pt, &plant) in self.grid.cells() {
            if visited.contains(&pt) {continue}

            let mut fences = FxHashSet::default();
            
            let points = flood_fill_adjacent(&pt, |&tile, &adj| {
                let belongs = self.grid.get(&adj).is_some_and(|&adj_plant| adj_plant == plant);
                if !belongs {
                    fences.insert((tile, Direction::try_from(adj - tile).unwrap()));
                }
                belongs
            }).collect_vec();
            visited.extend(points.iter());
            self.regions.push(Region {
                plant,
                points,
                fences,
            })
        } 
    }
}

impl Region {
    fn sides(&self) -> usize {
        let mut total = 0;
        let mut visited = FxHashSet::default();
        for fence in &self.fences {
            if visited.contains(fence) {continue}

            let (pos, direction) = *fence;
            
            let side = flood_fill(&pos,
                |&tile| direction.sides().into_iter().map(move |d| tile + d),
                |_, &adj| self.fences.contains(&(adj, direction))
            );
            visited.extend(side.into_iter().map(|pt| (pt, direction)));
            
            total += 1;
        }
        
        // println!("{}@{} has {total} sides", self.plant, self.points[0]);

        total
    }
}