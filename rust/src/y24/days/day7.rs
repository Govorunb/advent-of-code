use num::Integer;
use crate::*;

aoc_day!(
    day = 7,
    output = usize,
    examples = [
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 3749),
            (Self::INPUT, 850435817339),
        ],
        test_cases![
            (Self::EXAMPLES[0], 11387),
            (Self::INPUT, 104824810233437),
        ]
    ],
    solve = |input, part| {
        let lines = input.lines();
        let equations = lines.map(|l| {
            let (total_s, el_s) = l.split_once(':').unwrap();
            let total = total_s.parse().unwrap();
            let elements = el_s.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            Equation { total, elements }
        }).collect_vec();
        
        equations.iter()
            .filter(|&eqn| Self::search(eqn.total, &eqn.elements, part))
            .map(|eq| eq.total)
            .sum()
    }
);

#[derive(Debug, Clone)]
struct Equation {
    total: usize,
    elements: Vec<usize>,
}

impl Day7 {
    fn search(total: usize, elements: &[usize], part: Part) -> bool {
        if elements.len() == 1 {return elements[0] == total}
        
        // recursing backwards is faster (and also lets us use slices)
        // as opposed to checking forwards (where each i forces you to complete 3^i checks
        // (thereabouts, you can sometimes short-circuit maybe a couple places before the end))
        // when checking backwards, each operator validation failure is equivalent to 3^(i-1) checks
        // this discovery was unfortunately not mine - but it was still a very fun one
        let (&last, pop) = elements.split_last().unwrap();
        
        // plus
        if last <= total && Self::search(total - last, pop, part) {return true}
        
        // mul
        // if it doesn't divide cleanly, can't possibly be a mul
        if total % last == 0 && Self::search(total / last, pop, part) {return true}
        
        if let Part::Two = part {
            // concat
            let mut total_s = total.to_string();
            let last_s = last.to_string();
            if total_s.len() > last_s.len() && total_s.ends_with(&last_s) {
                total_s.truncate(total_s.len() - last_s.len());
                if Self::search(total_s.parse().unwrap(), pop, part) {return true}
            }
        }
        
        false
    }
}