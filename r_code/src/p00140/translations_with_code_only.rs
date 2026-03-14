use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut tei: [u8; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 5, 4, 3, 2, 1];
    let mut ans: [[&str; 20]; 1000] = Default::default(); // Placeholder for ans, not used in the original code

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let from: i32 = parts.next().unwrap().parse().unwrap();
        let to: i32 = parts.next().unwrap().parse().unwrap();

        if from < to {
            for j in from..to {
                print!("{} ", j);
            }
            println!("{}", to);
        } else if to == 5 {
            for j in from..=9 {
                print!("{} ", j);
            }
            println!("5");
        } else if from > to {
            if from >= 6 && from <= 9 {
                for j in from..=9 {
                    print!("{} ", j);
                }
                for j in (to + 1)..=5 {
                    print!("{} ", j);
                }
                println!("{}", to);
            } else {
                for j in (to + 1)..=from {
                    print!("{} ", j);
                }
                println!("{}", to);
            }
        }
    }
}
