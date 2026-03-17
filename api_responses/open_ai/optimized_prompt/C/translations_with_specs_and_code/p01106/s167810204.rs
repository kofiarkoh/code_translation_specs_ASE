use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        
        // Read n, i, j from input
        let n: usize = match parts.next().unwrap().parse() {
            Ok(num) if num > 0 => num,
            _ => break, // Terminate if n is non-positive
        };
        
        let mut i: i64 = parts.next().unwrap().parse().unwrap();
        let mut j: i64 = parts.next().unwrap().parse().unwrap();

        let mut low = vec![0; n];
        let mut ans = vec![' '; n]; // Initialize with spaces for clarity

        for k in 0..n {
            low[n - 1 - k] = ((i >> (n - 1 - k)) & 1) as i32; // Get the k-th bit from the end
            if low[n - 1 - k] == 0 {
                i = !i; // Bitwise NOT if the bit is 0
            }
        }

        for k in 0..n {
            ans[k] = if ((j >> (n - 1 - k)) & 1) == low[k] {
                'L'
            } else {
                'R'
            };
            if low[k] == 0 {
                j = !j; // Bitwise NOT if the corresponding low bit is 0
            }
        }

        // Print the result as a string
        println!("{}", ans.iter().collect::<String>());
    }
}
