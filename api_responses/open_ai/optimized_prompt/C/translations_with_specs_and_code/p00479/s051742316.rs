use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the total number of positions
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    // Read the number of pairs to process
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..k {
        // Read each pair of integers
        let pair: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (mut ai, mut bi) = (pair[0], pair[1]);

        // Adjust positions based on the total number of positions
        if n + 1 - ai < ai {
            ai = n + 1 - ai;
        }
        if n + 1 - bi < bi {
            bi = n + 1 - bi;
        }

        // Compute the value based on the adjusted positions
        let c = if ai > bi {
            (bi - 1) % 3 + 1
        } else {
            (ai - 1) % 3 + 1
        };

        // Output the computed value
        println!("{}", c);
    }
}
