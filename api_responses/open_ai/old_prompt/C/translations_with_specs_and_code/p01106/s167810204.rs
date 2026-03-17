use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let n: i32 = parts.next().unwrap().parse().unwrap();
        
        // Check for the precondition that n must be greater than 0
        if n <= 0 {
            break;
        }

        let mut i: i64 = parts.next().unwrap().parse().unwrap();
        let mut j: i64 = parts.next().unwrap().parse().unwrap();

        let mut ans = String::with_capacity(n as usize);
        let mut low = vec![false; n as usize];

        for k in 0..n {
            low[(n - 1 - k) as usize] = (i >> (n - 1 - k)) & 1 == 1;
            if !low[(n - 1 - k) as usize] {
                i = !i; // Bitwise NOT operation
            }
        }

        for k in 0..n {
            ans.push(if (j >> (n - 1 - k) & 1 == 1) == low[k as usize] {
                'L'
            } else {
                'R'
            });
            if !low[k as usize] {
                j = !j; // Bitwise NOT operation
            }
        }

        // Output the result
        println!("{}", ans);
    }
}
