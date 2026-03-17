use std::io::{self, BufRead};

fn main() {
    let mapping: [[Option<usize>; 10]; 10] = [
        [Some(1), Some(2), None, None, None, None, None, None, None, None],
        [Some(3), None, None, None, None, None, None, None, None, None],
        [Some(4), None, None, None, None, None, None, None, None, None],
        [Some(5), None, None, None, None, None, None, None, None, None],
        [Some(6), None, None, None, None, None, None, None, None, None],
        [Some(7), None, None, None, None, None, None, None, None, None],
        [Some(8), None, None, None, None, None, None, None, None, None],
        [Some(9), None, None, None, None, None, None, None, None, None],
        [Some(0), None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None],
    ];

    const TARGET_CITY: usize = 5;

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let input = line.unwrap();

        if input.starts_with('#') {
            break;
        }

        if input.len() > 10 {
            println!("No");
            continue;
        }

        let mut current_city = 0;
        let mut valid = true;

        for ch in input.chars() {
            if let Some(digit) = ch.to_digit(10) {
                let digit = digit as usize;

                if let Some(next_city) = mapping[current_city][digit] {
                    current_city = next_city;
                } else {
                    valid = false;
                    break;
                }
            } else {
                valid = false;
                break;
            }
        }

        if valid && current_city == TARGET_CITY {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
