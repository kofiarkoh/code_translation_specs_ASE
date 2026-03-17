use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let mut parts = input.split_whitespace();
        let n: i32 = parts.next().unwrap().parse().unwrap();
        
        // Check for the termination condition
        if n <= 0 {
            break;
        }
        
        let mut i: i64 = parts.next().unwrap().parse().unwrap();
        let mut j: i64 = parts.next().unwrap().parse().unwrap();
        
        let mut ans = String::with_capacity(n as usize);
        let mut low = vec![0; n as usize];

        // Process the first integer i
        for k in 0..n {
            low[(n - 1 - k) as usize] = ((i >> (n - 1 - k)) & 1) as i32;
            if low[(n - 1 - k) as usize] == 0 {
                i = !i; // Bitwise NOT operation
            }
        }

        // Process the second integer j
        for k in 0..n {
            let direction = if ((j >> (n - 1 - k)) & 1) == low[k as usize] as i64 {
                'L'
            } else {
                'R'
            };
            ans.push(direction);
            if low[k as usize] == 0 {
                j = !j; // Bitwise NOT operation
            }
        }

        // Output the result
        println!("{}", ans);
        
        // Clear input for the next iteration
        input.clear();
    }
}
