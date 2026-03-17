fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: i32 = line.trim().parse().unwrap();

        // Terminate if n is zero
        if n == 0 {
            break;
        }

        // Read the sequence of integers
        let mut go: Vec<i32> = Vec::new();
        for _ in 0..n {
            let num: i32 = reader.lines().next().unwrap().unwrap().trim().parse().unwrap();
            go.push(num);
        }

        // Process the sequence according to the specified rules
        for i in 0..go.len() {
            if i > 0 && i < go.len() - 1 {
                // If the current element is not zero and both neighbors are zero, set it to zero
                if go[i] != 0 && go[i - 1] == 0 && go[i + 1] == 0 {
                    go[i] = 0;
                }
            }
        }

        // Count the number of zeros in the modified array
        let zero_count = go.iter().filter(|&&x| x == 0).count();

        // Output the count of zeros
        println!("{}", zero_count);
    }
}
