use crate::*;

pub const DAY9_EXAMPLE: &str =
"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

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
            let from = cap.name("from").unwrap().as_str().to_string();
            let to = cap.name("to").unwrap().as_str().to_string();
            let dist = cap.name("dist").unwrap().as_str().parse().unwrap();
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

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY9_EXAMPLE, 605),
                (self.input(), 207),
            ],
            test_cases![
                (DAY9_EXAMPLE, 982),
                (self.input(), 804),
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
    
    fn tsp_<'a>(costs: &FxHashMap<(String, String), usize>, unvisited: &FxHashSet<String>, best: &mut Tour, curr_route: Vec<String>, target: &String, check: fn(item: usize, cost: usize) -> bool) {
        for curr_city in unvisited {
            let mut next_unvisited = unvisited.clone();
            next_unvisited.remove(curr_city);
            let mut next_route = curr_route.clone();
            next_route.push(curr_city.clone());
            
            if next_unvisited.is_empty() {
                next_route.push(target.clone());
                
                let cost = Self::route_cost(costs, &next_route);
                // .is_none_or
                if best.distance.map_or(true, |x| check(x, cost)) {
                    best.route = next_route.clone();
                    best.distance = Some(cost);
                }
                
                return;
            } else {
                Self::tsp_(costs, &next_unvisited, best, next_route, target, check)
            }
        }
    }
    
    fn route_cost(graph: &FxHashMap<(String, String), usize>, route: &Vec<String>) -> usize {
        // println!("route: {:?}", route);
        route.iter()
            .tuple_windows()
            .map(|(from, to)| graph.get(&(from.clone(), to.clone())).unwrap())
            .sum()
    }
}