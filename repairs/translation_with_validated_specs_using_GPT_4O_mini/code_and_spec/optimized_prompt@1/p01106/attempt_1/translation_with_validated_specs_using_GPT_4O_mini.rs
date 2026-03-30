use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    while stdin.read_line(&mut input).unwrap() > 0 {
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();

        // Check precondition for n
        if n == 0 {
            break;
        }

        let mut i: i64 = parts.next().unwrap().parse().unwrap();
        let mut j: i64 = parts.next().unwrap().parse().unwrap();

        i -= 1; // Decrement i as per the C code
        j -= 1; // Decrement j as per the C code

        let mut low = vec![0; n]; // Vector to hold the low bits
        let mut ans = vec![' '; n]; // Vector to hold the answer characters

        for k in 0..n {
            low[n - 1 - k] = ((i >> (n - 1 - k)) & 1) as i32; // Extracting bits
            if low[n - 1 - k] == 0 {
                i = !i; // Bitwise negation if low bit is 0
            }
        }

        for k in 0..n {
            ans[k] = if ((j >> (n - 1 - k)) & 1) == low[k] {
                'L'
            } else {
                'R'
            };
            if low[k] == 0 {
                j = !j; // Bitwise negation if low bit is 0
            }
        }

        // Output the result
        println!("{}", ans.iter().collect::<String>());
        
        // Clear input for the next iteration
        input.clear();
    }
}
