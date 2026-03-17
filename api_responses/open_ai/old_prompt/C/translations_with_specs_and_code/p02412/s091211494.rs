use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());

        let n = match nums.next() {
            Some(num) => num,
            None => continue,
        };
        let x = match nums.next() {
            Some(num) => num,
            None => continue,
        };

        if n == 0 && x == 0 {
            break;
        }

        let mut count = 0;

        // Iterate over all unique triplets (i, j, k)
        for k in 1..=n {
            for j in 1..=n {
                for i in 1..=n {
                    // Ensure i, j, k are distinct
                    if i != j && j != k && i != k {
                        if i + j + k == x {
                            count += 1;
                        }
                    }
                }
            }
        }

        // Each triplet (i, j, k) is counted 6 times (permutations), so divide by 6
        println!("{}", count / 6);
    }
}
