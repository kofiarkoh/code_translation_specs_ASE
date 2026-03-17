use std::io::{self, BufRead};

fn count_four_square_representations(n: u32) -> u32 {
    let mut count = 0;
    let limit = (n as f64).sqrt() as u32;

    for a in 0..=limit {
        for b in 0..=limit {
            for c in 0..=limit {
                for d in 0..=limit {
                    if a * a + b * b + c * c + d * d == n {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let input = line.unwrap().trim().parse::<u32>();
        match input {
            Ok(n) if n > 0 => {
                let representations = count_four_square_representations(n);
                println!("{}", representations);
            }
            Ok(0) => break, // Terminate on zero input
            _ => {} // Assumes all inputs are valid as per the specification
        }
    }
}
