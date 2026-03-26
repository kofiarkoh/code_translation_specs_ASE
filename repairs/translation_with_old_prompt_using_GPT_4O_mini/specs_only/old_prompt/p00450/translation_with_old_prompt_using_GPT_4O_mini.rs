use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    loop {
        let mut n_str = String::new();
        input.read_line(&mut n_str).unwrap();
        let n: i32 = n_str.trim().parse().unwrap();

        // Terminate if n is zero
        if n == 0 {
            break;
        }

        let mut go = Vec::with_capacity(n as usize);
        let mut line = String::new();
        input.read_line(&mut line).unwrap();
        for num_str in line.trim().split_whitespace() {
            let num: i32 = num_str.parse().unwrap();
            go.push(num);
        }

        // Process the go array according to the specified rules
        for i in 0..n as usize {
            if i > 0 && i < (n - 1) as usize {
                // If the current element is not zero and both neighbors are zero, set it to zero
                if go[i] != 0 && go[i - 1] == 0 && go[i + 1] == 0 {
                    go[i] = 0;
                }
            }
        }

        // Count the number of zeros in the modified go array
        let zero_count = go.iter().filter(|&&x| x == 0).count();
        println!("{}", zero_count);
    }
}
