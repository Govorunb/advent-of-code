use crate::*;

aoc_day!(
    day = 11,
    output = String,
    examples = [""],
    tests = [
        test_cases![
            ("abcdefgh", "abcdffaa".to_string()),
            ("ghijklmn", "ghjaabcc".to_string()),
            // (Self::INPUT, 0),
        ],
        test_cases![
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
        let starting_pass: &[u8;8] = input.as_bytes().try_into().unwrap();
        
        let mut pass = Vec::from(starting_pass);
        // do while
        loop {
            Self::increment(&mut pass);
            if Self::is_valid(&pass) {break}
        }
        match part {
            Part::One => {
                String::from_utf8(pass).unwrap()
            },
            Part::Two => {
                loop { // again
                    Self::increment(&mut pass);
                    if Self::is_valid(&pass) {break}
                }
                String::from_utf8(pass).unwrap()
            }
        }
    }
);


impl Day11 {
    fn increment(x: &mut [u8]) {
        let mut i = x.len()-1;
        x[i] += 1;
        while x[i] > b'z' {
            x[i] = b'a';
            i -= 1;
            x[i] += 1;
        }
    }
    
    fn is_valid(x: &[u8]) -> bool {
        Self::rule1(x) && Self::rule2(x) && Self::rule3(x)
    }
    
    fn rule1(x: &[u8]) -> bool {
        x.iter().tuple_windows().any(|(&a, &b, &c)| b == a+1 && c == a+2)
    }
    
    fn rule2(x: &[u8]) -> bool {
        x.iter().all(|&c| c != b'i' && c != b'l' && c != b'o')
    }
    fn rule3(x: &[u8]) -> bool {
        let mut doubles = 0;
        let mut last_double: Option<(u8, usize)> = None;
        for i in 0..x.len()-1 {
            let c = x[i];
            if x[i+1] == c && last_double.is_none_or(|(d, pos)| d != c && pos+1 < i) {
                doubles += 1;
                last_double = Some((c, i));
            }
        }
        
        doubles >= 2
    }
}
