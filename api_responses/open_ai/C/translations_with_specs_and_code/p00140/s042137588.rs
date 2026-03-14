use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of pairs
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input for number of pairs");

    for _ in 0..n {
        // Read each pair of integers
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().expect("Invalid input for integers"));
        let from = nums.next().unwrap();
        let to = nums.next().unwrap();

        if from < to {
            // Print the sequence from 'from' to 'to'
            for j in from..to {
                print!("{} ", j);
            }
            println!("{}", to);
        } else if to == 5 {
            // Special case when 'to' is 5
            for j in from..=9 {
                print!("{} ", j);
            }
            println!("5");
        } else if from > to {
            if from >= 6 && from <= 9 {
                // Print from 'from' to 9, then from 5 down to 'to'
                for j in from..=9 {
                    print!("{} ", j);
                }
                for j in (to + 1)..=5 {
                    print!("{} ", j);
                }
                println!("{}", to);
            } else {
                // Print from 'from' down to 'to'
                for j in (to..from).rev() {
                    print!("{} ", j);
                }
                println!("{}", to);
            }
        }
    }
}
