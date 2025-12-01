use std::{ascii::Char, collections::HashSet};

use rustc_hash::FxHasher;

use crate::*;

aoc_day!(
    day = 7,
    output = usize,
    examples = [],
    tests = [
        test_cases![
            ("abba[mnop]qrst", 1),
            ("abcd[bddb]xyyx", 0),
            ("aaaa[qwer]tyui", 0),
            ("ioxxoj[asdfgh]zxcvbn", 1),
            (Self::INPUT, 115),
        ],
        test_cases![
            ("aba[bab]xyz", 1),
            ("xyx[xyx]xyx", 0),
            ("aaa[kek]eke", 1),
            ("zazbz[bzb]cdb", 1),
            (Self::INPUT, 231),
        ]
    ],
    solve = |input, part| {
        let lines = input.lines();
        match part {
            Part::One => {
                lines.map(check_tls).filter(|&x| x).count()
            },
            Part::Two => {
                lines.map(check_ssl).filter(|&x| x).count()
            }
        }
    }
);

fn check_tls(ip: &str) -> bool {
    let mut have_abba = false;
    let mut hyper = false;
    let s = ip.as_ascii().unwrap();
    for (&a,&b,&c,&d) in s.iter().tuple_windows() {
        if [a,b,c,d].into_iter().contains(&Char::LeftSquareBracket) {
            hyper = true;
            continue;
        }
        if [a,b,c,d].into_iter().contains(&Char::RightSquareBracket) {
            hyper = false;
            continue;
        }
        if hyper || !have_abba {
            if a == d && b == c && a != b {
                if hyper {
                    return false;
                }
                have_abba = true;
            }
        }
    }
    have_abba
}

fn check_ssl(ip: &str) -> bool {
    let mut abas: FxHashSet<(Char, Char)> = Default::default();
    let mut babs: FxHashSet<(Char, Char)> = Default::default();
    let mut hyper = false;
    let s = ip.as_ascii().unwrap();
    for (&x, &y, &z) in s.iter().tuple_windows() {
        if [x,y,z].into_iter().contains(&Char::LeftSquareBracket) {
            hyper = true;
            continue;
        }
        if [x,y,z].into_iter().contains(&Char::RightSquareBracket) {
            hyper = false;
            continue;
        }
        if x == z && x != y {
            let (this, other) = if hyper {(&mut babs, &mut abas)} else {(&mut abas, &mut babs)};
            this.insert((x,y));
            if other.contains(&(y, x)) {
                return true;
            }
        }
    }
    false
}
