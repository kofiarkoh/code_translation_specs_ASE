use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = nums.next().unwrap();
        let x = nums.next().unwrap();

        // Termination condition
        if n == 0 && x == 0 {
            break;
        }

        let mut count = 0;

        // Iterate through all combinations of three distinct integers
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

        // Output the count divided by 6
        println!("{}", count / 6);
    }
}
