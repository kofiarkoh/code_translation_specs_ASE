use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut str_buf = String::new();

    for line in handle.lines() {
        let input = line.unwrap();
        let str_slice = &input[..std::cmp::min(input.len(), 10)]; // Limit to 10 characters
        let mut i = 9; // Default to 9

        if str_slice.len() < 3 {
            continue; // Skip if input is too short
        }

        if str_slice.chars().nth(0).unwrap() != 's' && 
           str_slice.chars().nth(0) == str_slice.chars().nth(1) && 
           str_slice.chars().nth(1) == str_slice.chars().nth(2) {
            i = 0;
        } else if str_slice.chars().nth(0) != 's' && 
                  str_slice.chars().nth(0) == str_slice.chars().nth(3) && 
                  str_slice.chars().nth(3) == str_slice.chars().nth(6) {
            i = 0;
        } else if str_slice.chars().nth(0) != 's' && 
                  str_slice.chars().nth(0) == str_slice.chars().nth(4) && 
                  str_slice.chars().nth(4) == str_slice.chars().nth(8) {
            i = 0;
        } else {
            if str_slice.len() > 4 && 
               str_slice.chars().nth(1) != 's' && 
               str_slice.chars().nth(1) == str_slice.chars().nth(4) && 
               str_slice.chars().nth(4) == str_slice.chars().nth(7) {
                i = 1;
            } else {
                if str_slice.len() > 5 && 
                   str_slice.chars().nth(2) != 's' && 
                   str_slice.chars().nth(2) == str_slice.chars().nth(5) && 
                   str_slice.chars().nth(5) == str_slice.chars().nth(8) {
                    i = 2;
                } else if str_slice.len() > 6 && 
                          str_slice.chars().nth(2) != 's' && 
                          str_slice.chars().nth(2) == str_slice.chars().nth(4) && 
                          str_slice.chars().nth(4) == str_slice.chars().nth(6) {
                    i = 2;
                } else {
                    if str_slice.len() > 5 && 
                       str_slice.chars().nth(3) != 's' && 
                       str_slice.chars().nth(3) == str_slice.chars().nth(4) && 
                       str_slice.chars().nth(4) == str_slice.chars().nth(5) {
                        i = 3;
                    } else {
                        if str_slice.len() > 8 && 
                           str_slice.chars().nth(6) != 's' && 
                           str_slice.chars().nth(6) == str_slice.chars().nth(7) && 
                           str_slice.chars().nth(7) == str_slice.chars().nth(8) {
                            i = 6;
                        } else {
                            i = 9;
                        }
                    }
                }
            }
        }

        if i == 9 {
            println!("d");
        } else {
            if str_slice.chars().nth(i).unwrap() == 'o' {
                println!("o");
            } else {
                println!("x");
            }
        }
    }
}
