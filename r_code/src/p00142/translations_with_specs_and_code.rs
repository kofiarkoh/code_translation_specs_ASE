use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut exp = vec![0; 10001];
    let mut count = vec![0; 6000];

    // Precompute squares
    for i in 1..10001 {
        exp[i] = i * i;
    }

    for line in handle.lines() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break; // Terminate on zero input
        }

        // Reset count for each n
        count.fill(0);
        let mut mod_values = vec![0; n];
        let mut distinct_squares = Vec::new();

        // Compute mod values
        for i in 1..n {
            mod_values[i] = exp[i] % n;
        }

        // Collect distinct mod values
        for &value in &mod_values[1..n] {
            if !distinct_squares.contains(&value) {
                distinct_squares.push(value);
            }
        }

        let h = (n - 1) / 2;

        // Count distinct differences
        let c = distinct_squares.len();
        for i in 0..c {
            for j in 0..c {
                if distinct_squares[i] != distinct_squares[j] {
                    let mut d = distinct_squares[i] as isize - distinct_squares[j] as isize;
                    if d < 0 {
                        d += n as isize;
                    }
                    if d > h as isize {
                        d = n as isize - d;
                    }
                    count[d as usize] += 1;
                }
            }
        }

        // Output results
        for i in 1..=h {
            println!("{}", count[i]);
        }
    }
}
