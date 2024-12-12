use crate::*;

pub struct Day9 {
    
}

impl Day<9> for Day9 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day9.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        // tsp... sigh
        let mut edges: FxHashMap<(String, String), usize> = FxHashMap::default();
        let mut cities: FxHashSet<String> = FxHashSet::default();
        let regex: Regex = Regex::new(r#"(?<from>\w+) to (?<to>\w+) = (?<dist>\d+)"#).unwrap();
        for cap in regex.captures_iter(input) {
            let from: String = cap.parse("from");
            let to: String = cap.parse("to");
            let dist = cap.parse("dist");
            cities.insert(from.clone());
            cities.insert(to.clone());
            edges.insert((from.clone(), to.clone()), dist);
            edges.insert((to, from), dist);
        }
        // not worth rearchitecting for rayon
        match part {
            Part::One => {
                let mut distances: FxHashMap<(String, String), usize> = FxHashMap::default();
                for start in cities.iter() {
                    for end in cities.iter() {
                        if start == end {continue}
                        if distances.contains_key(&(start.clone(), end.clone()))
                            || distances.contains_key(&(end.clone(), start.clone())) {
                            continue
                        }
                        let tour = Self::tsp(&edges, &cities, start, end, |i, c| c < i);
                        
                        distances.insert((start.clone(), end.clone()), tour.distance.unwrap());
                        distances.insert((end.clone(), start.clone()), tour.distance.unwrap());
                    }
                }
                
                *distances.values().min().unwrap()
            },
            Part::Two => {
                let mut distances: FxHashMap<(String, String), usize> = FxHashMap::default();
                for start in cities.iter() {
                    for end in cities.iter() {
                        if start == end {continue}
                        if distances.contains_key(&(start.clone(), end.clone()))
                            || distances.contains_key(&(end.clone(), start.clone())) {
                            continue
                        }
                        let tour = Self::tsp(&edges, &cities, start, end, |i, c| c > i);

                        distances.insert((start.clone(), end.clone()), tour.distance.unwrap());
                        distances.insert((end.clone(), start.clone()), tour.distance.unwrap());
                    }
                }

                *distances.values().max().unwrap()
            }
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 605),
                (Self::INPUT, 207),
            ],
            test_cases![
                (Self::EXAMPLES[0], 982),
                (Self::INPUT, 804),
            ]
        ]
    }
}

impl Default for Day9 {
    fn default() -> Self {
        Self::new()
    }
}

struct Tour {
    pub distance: Option<usize>,
    pub route: Vec<String>,
}

impl Day9 {
    pub fn new() -> Self {
        Self {}
    }
    
    fn tsp(costs: &FxHashMap<(String, String), usize>, cities: &FxHashSet<String>, start: &String, end: &String, check: fn(item: usize, cost: usize) -> bool) -> Tour {
        let starting_route = vec![start.clone()];
        let mut tour: Tour = Tour {distance: None, route: starting_route.clone()};
        
        let mut to_visit = cities.clone();
        to_visit.remove(start);
        to_visit.remove(end);
        
        Self::tsp_(costs, &to_visit, &mut tour, starting_route, end, check);
        
        tour
    }
    
    fn tsp_(costs: &FxHashMap<(String, String), usize>, unvisited: &FxHashSet<String>, best: &mut Tour, curr_route: Vec<String>, target: &String, check: fn(item: usize, cost: usize) -> bool) {
        for curr_city in unvisited {
            let mut next_unvisited = unvisited.clone();
            next_unvisited.remove(curr_city);
            let mut next_route = curr_route.clone();
            next_route.push(curr_city.clone());
            
            if next_unvisited.is_empty() {
                next_route.push(target.clone());
                
                let cost = Self::route_cost(costs, &next_route);
                if best.distance.is_none_or(|x| check(x, cost)) {
                    best.route = next_route.clone();
                    best.distance = Some(cost);
                }
                
                return;
            } else {
                Self::tsp_(costs, &next_unvisited, best, next_route, target, check)
            }
        }
    }
    
    fn route_cost(graph: &FxHashMap<(String, String), usize>, route: &[String]) -> usize {
        // println!("route: {:?}", route);
        route.iter()
            .tuple_windows()
            .map(|(from, to)| graph.get(&(from.clone(), to.clone())).unwrap())
            .sum()
    }
}