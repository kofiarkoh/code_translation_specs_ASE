use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n
    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    // Read k
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..k {
        // Read ai and bi
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        let mut ai: i32 = nums.next().unwrap().parse().unwrap();
        let mut bi: i32 = nums.next().unwrap().parse().unwrap();

        // Adjust ai and bi if they exceed half of n
        if n + 1 - ai < ai {
            ai = n + 1 - ai;
        }
        if n + 1 - bi < bi {
            bi = n + 1 - bi;
        }

        // Compute the result based on adjusted ai and bi
        let c = if ai > bi {
            (bi - 1) % 3 + 1
        } else {
            (ai - 1) % 3 + 1
        };

        // Output the result
        println!("{}", c);
    }
}
