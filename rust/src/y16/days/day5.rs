use std::ops::IndexMut;

use crate::*;

aoc_day!(
    day = 5,
    output = String,
    examples = ["abc"],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], "18f47a30".into()),
            (Self::INPUT, "f77a0e6e".into()),
        ],
        test_cases![
            (Self::EXAMPLES[0], "05ace8e3".into()),
            (Self::INPUT, "999828ec".into()),
        ]
    ],
    solve = |input, part| {
        let door_id = input;
        match part {
            Part::One => {
                let mut password = String::with_capacity(8);
                let base_len = door_id.len();
                let mut buf = String::with_capacity(base_len + 10);
                buf.push_str(door_id);
                let mut i = 0;
                for _ in 0..8usize {
                    loop {
                        buf.truncate(base_len);
                        buf.push_str(&i.to_string());
                        let hash = md5::compute(buf.as_bytes());
                        i += 1;
                        // 5 zeroes
                        if hash.starts_with(&[00, 00]) && hash[2] < 0x10 {
                            let c = format!("{hash:?}").chars().nth(5).unwrap();
                            if !cfg!(test) {
                                println!("found hash {hash:?} from {buf} - 6th is {c}");
                            }
                            password.push(c);
                            break;
                        }
                    }
                }
                password
            },
            Part::Two => {
                let mut password = ['\0'; 8];
                let base_len = door_id.len();
                let mut buf = String::with_capacity(base_len + 10);
                buf.push_str(door_id);
                let mut i = 0;
                loop {
                    loop {
                        buf.truncate(base_len);
                        buf.push_str(&i.to_string());
                        let hash = md5::compute(buf.as_bytes());
                        i += 1;
                        // 5 zeroes, 6th <= 7
                        if hash.starts_with(&[00, 00]) && hash[2] < 8 {
                            let [pos, c] = format!("{hash:?}").chars().skip(5).next_array().unwrap();
                            let pos = pos.to_digit(10).unwrap() as usize;
                            if !cfg!(test) {
                                println!("found hash {hash:?} from {buf} - {c} into {pos}; {}", if password[pos] == '\0' {"free"} else {"taken"});
                            }
                            if password[pos] == '\0' {
                                password[pos] = c;
                                break;
                            }
                        }
                    }
                    if password.iter().all(|&c| c != '\0') {
                        break;
                    }
                }
                password.iter().collect()
            }
        }
    }
);
