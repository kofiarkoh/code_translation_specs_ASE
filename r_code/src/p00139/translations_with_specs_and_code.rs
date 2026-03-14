use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of strings to process
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input");

    // Ensure n is greater than zero as per the specification
    if n == 0 {
        return; // Behavior is not defined for n <= 0
    }

    for _ in 0..n {
        // Read each string
        let input = lines.next().unwrap().unwrap();
        
        // Ensure the string does not exceed 200 characters
        if input.len() > 200 {
            continue; // Behavior is not defined for malformed input
        }

        let mut cnt1 = 0;
        let mut cnt2 = 0;
        let mut flag = false;

        if input.starts_with('>') {
            if input.chars().nth(1) == Some('\'') {
                // Count '=' characters before the first '#'
                for j in 2..input.len() {
                    if input.chars().nth(j) == Some('=') {
                        cnt1 += 1;
                    }
                    if input.chars().nth(j + 1) == Some('#') {
                        break;
                    }
                }
                // Count '=' characters after the first '#'
                for j in cnt1 + 2..input.len() {
                    if input.chars().nth(j) == Some('=') {
                        cnt2 += 1;
                    }
                }
                // Check the conditions for pattern A
                if cnt1 == cnt2 && input.chars().nth(cnt1 + cnt2 + 3) == Some('~') && input.chars().nth(cnt1 + cnt2 + 4).is_none() {
                    println!("A");
                    flag = true;
                }
            } else if input.chars().nth(1) == Some('^') {
                // Check for pattern B
                for j in 2..input.len() {
                    if j % 2 == 0 {
                        if input.chars().nth(j) == Some('Q') && input.chars().nth(j + 1) == Some('=') {
                            if input.chars().nth(j + 2) == Some('~') && input.chars().nth(j + 3) == Some('~') && input.chars().nth(j + 4).is_none() {
                                println!("B");
                                flag = true;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        if !flag {
            println!("NA");
        }
    }
}
