fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first input: total number of elements (n)
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input for n");
    // Read the second input: number of pairs to process (k)
    let k: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input for k");

    for _ in 0..k {
        // Read each pair of integers (ai, bi)
        let pair: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for pair"))
            .collect();

        let (mut ai, mut bi) = (pair[0], pair[1]);

        // Adjust ai and bi based on the specification
        if n + 1 - ai < ai {
            ai = n + 1 - ai;
        }
        if n + 1 - bi < bi {
            bi = n + 1 - bi;
        }

        // Compute the output value based on the adjusted values
        let c = if ai > bi {
            (bi - 1) % 3 + 1
        } else {
            (ai - 1) % 3 + 1
        };

        // Output the computed value
        println!("{}", c);
    }
}
