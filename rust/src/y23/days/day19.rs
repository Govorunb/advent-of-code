use crate::test_cases;
use crate::common::*;

use std::ops::Range;

pub struct Day19 {
    
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
enum Category {
    x, m, a, s
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ConditionType {
    Under(usize),
    Over(usize),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Send {
    Accept,
    Reject,
    Workflow(String),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Rule {
    cat: Category,
    cond: ConditionType,
    send: Send,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Workflow {
    name: String,
    cases: Vec<Rule>,
    default: Send,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MetalPart {
    metrics: [usize; 4],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AcceptableRanges {
    ranges: [Range<usize>; 4],
}

impl From<&str> for MetalPart {
    fn from(input: &str) -> Self {
        let (x,m,a,s) = input.trim_matches(['{', '}'])
            .splitn(4, ',')
            .map(|section| section[2..].parse::<usize>().unwrap())
            .collect_tuple().unwrap();

        Self {metrics: [x,m,a,s]}
    }
}

impl From<char> for Category {
    fn from(c: char) -> Self {
        match c {
            'x' => Category::x,
            'm' => Category::m,
            'a' => Category::a,
            's' => Category::s,
            _ => unreachable!()
        }
    }
}

impl From<&str> for ConditionType {
    fn from(input: &str) -> Self {
        let mut chars = input.chars();
        let sign = chars.next().unwrap() == '<';
        let num = chars.collect::<String>().parse::<usize>().unwrap();
        
        if sign { Self::Under(num) } else { Self::Over(num) }
    }
}

impl From<&str> for Send {
    fn from(input: &str) -> Self {
        match input.chars().next().unwrap() {
            'A' => Send::Accept,
            'R' => Send::Reject,
            _ => Send::Workflow(input.to_owned())
        }
    }
}

impl From<&str> for Rule {
    fn from(input: &str) -> Self {
        let cat = input.chars().next().unwrap().into();
        let (threshold_str, send_str) = input[1..].split_once(':').unwrap();
        let threshold = threshold_str.into();
        let send = send_str.into();
        Self {
            cat,
            cond: threshold,
            send,
        }
    }
}

impl From<&str> for Workflow {
    fn from(input: &str) -> Self {
        let (name, rest) = input.trim_end_matches('}').split_once('{').unwrap();
        let mut inner = rest.split(',');

        let default = inner.next_back().unwrap().into();
        let mut cases = inner.map_into::<Rule>().collect_vec();
        // remove duplicates like {x<1:R,R}
        for i in cases.len()..0 {
            // because order matters, once any send is different, nothing afterwards can be a dupe
            if cases[i].send != default {
                break;
            }
            cases.remove(i);
        }

        Self {
            name: name.to_owned(),
            default,
            cases,
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}:{}", self.cat, self.cond, self.send)?;
        Ok(())
    }
}
impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Category::x => 'x',
            Category::m => 'm',
            Category::a => 'a',
            Category::s => 's',
        })?;
        Ok(())
    }
}

impl Display for ConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ConditionType::Over(over) => format!(">{over}"),
            ConditionType::Under(under) => format!("<{under}"),
        })?;
        Ok(())
    }
}

impl Display for Send {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Send::Accept => "A",
            Send::Reject => "R",
            Send::Workflow(next) => next.as_str(),
        })?;
        Ok(())
    }
}

impl Display for AcceptableRanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.ranges)?;
        Ok(())
    }
}

impl Rule {
    fn matches(&self, part: MetalPart) -> bool {
        let value = part.metrics[self.cat as usize];
        self.cond.covers(value)
    }
}

impl ConditionType {
    fn covers(self, value: usize) -> bool {
        match self {
            ConditionType::Under(under) => value < under,
            ConditionType::Over(over) => value > over,
        }
    }
    fn invert(&self) -> ConditionType {
        match self {
            ConditionType::Over(over) => ConditionType::Under(over+1),
            ConditionType::Under(under) => ConditionType::Over(under-1),
        }
    }
}

impl Workflow {
    fn process(&self, part: MetalPart) -> &Send {
        for rule in &self.cases {
            if rule.matches(part) {
                return &rule.send;
            }
        }
        &self.default
    }

    fn count_combinations(&self, ranges: AcceptableRanges, workflows: &FxIndexMap<String, Workflow>) -> usize {
        let mut combinations = 0;
        let mut curr_ranges = ranges;
        for case in &self.cases {
            let case_ranges = curr_ranges.filter(case, true);
            combinations += match &case.send {
                Send::Workflow(next) => workflows[next.as_str()].count_combinations(case_ranges, workflows),
                Send::Accept => case_ranges.count(),
                Send::Reject => 0,
            };
            // subsequent cases can only be reached when the rule condition above fails
            curr_ranges = curr_ranges.filter(case, false);
        }

        combinations += match &self.default {
            Send::Workflow(next) => workflows[next.as_str()].count_combinations(curr_ranges.clone(), workflows),
            Send::Accept => curr_ranges.count(),
            Send::Reject => 0,
        };

        combinations
    }
}
impl MetalPart {
    fn score(self) -> usize {
        self.metrics.iter().sum()
    }
}

impl AcceptableRanges {
    fn filter(&self, rule: &Rule, pass_or_fail: bool) -> Self {
        let mut ranges = self.ranges.clone();
        let range = &mut ranges[rule.cat as usize];
        Self::filter_range(range, if pass_or_fail {rule.cond} else {rule.cond.invert()});
        Self { ranges }
    }
    
    fn filter_range(range: &mut Range<usize>, cond: ConditionType) {
        match cond {
            ConditionType::Over(over) => range.start = range.start.max(over + 1),
            ConditionType::Under(under) => range.end = range.end.min(under),
        }
    }

    fn count(&self) -> usize {
        self.ranges.iter()
            .map(ExactSizeIterator::len)
            .reduce(|acc, l| acc * l)
            .unwrap()
    }
}

impl Day<19> for Day19 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day19.txt");

    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let newlines_fixed = input.replace("\r\n", "\n");
        let (workflows_str, parts_str) = newlines_fixed
            .split_once("\n\n").unwrap_or_else(|| {
                panic!("{}", newlines_fixed);
            });
        let workflows: FxIndexMap<String, Workflow> = workflows_str.lines()
            .map(str::trim)
            .map_into::<Workflow>()
            .map(|wf| (wf.name.clone(), wf))
            .collect();
        let parts = parts_str.lines()
            .map(str::trim)
            .map_into::<MetalPart>()
            .collect_vec();
        let start = &workflows["in"];
        match part {
            Part::One => {
                let (accepted, _rejected): (Vec<MetalPart>, Vec<MetalPart>) = parts.iter()
                    .partition(|&&part| {
                        let mut curr = &Send::Workflow(start.name.to_owned());
                        while let Send::Workflow(wf_name) = curr {
                            let wf = &workflows[wf_name];
                            curr = wf.process(part);
                        }
                        matches!(curr, Send::Accept)
                    });
                accepted.into_iter()
                    .map(MetalPart::score)
                    .sum()
            },
            Part::Two => {
                let ranges = AcceptableRanges {
                    ranges: [1..4001, 1..4001, 1..4001, 1..4001], // no copy...
                };

                start.count_combinations(ranges, &workflows)
            }
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 19114),
                (Self::INPUT, 487623),
            ],
            test_cases![
                // simplest possible case
                ("in{x<20:A,R}\n\n{x=0,m=0,a=0,s=0}", 19*4000*4000*4000),
                // same as above, flipping accept/reject
                ("in{x>20:R,A}\n\n{x=0,m=0,a=0,s=0}", 20*4000*4000*4000),
                // a second category
                ("in{x<20:b,R}\nb{m>1999:R,A}\n\n{x=0,m=0,a=0,s=0}", 19*1999*4000*4000),
                // further indirection + redundant rules
                ("in{x<20:b,x>500:c,d}
                  b{m>1999:R,A}
                  c{m>500:A,R}
                  d{x>1000:A,b}

                {x=0,m=0,a=0,s=0}", (
                    19*1999              // in (x<20) * b (m<=1999)
                  +(4000-500)*(4000-500) // in (x>500) * c (m>500)
                  +(500-19)*1999         // in (20<=x<=500) * d (->b) * b (m<=1999)
                )*4000*4000),
                (Self::EXAMPLES[0], 167409079868000),
                (Self::INPUT, 113550238315130),
            ]
        ]
    }
}

impl Default for Day19 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day19 {
    pub fn new() -> Self {
        Self {
        }
    }
}