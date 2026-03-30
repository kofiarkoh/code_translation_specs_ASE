use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of strings N
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Create a vector to hold the strings s
    let mut s: Vec<String> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        s.push(input.trim().to_string());
    }

    // Read the number of strings M
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().unwrap();

    // Create a vector to hold the strings t
    let mut t: Vec<String> = Vec::with_capacity(m);
    for _ in 0..m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        t.push(input.trim().to_string());
    }

    let mut max = 0;

    for i in 0..n {
        let mut count = 0;

        // Count occurrences of s[i] in s
        for j in 0..n {
            if s[i] == s[j] {
                count += 1;
            }
        }

        // Subtract occurrences of s[i] in t
        for j in 0..m {
            if s[i] == t[j] {
                count -= 1;
            }
        }

        // Update max if the current count is greater
        if count > max {
            max = count;
        }
    }

    // Print the result
    println!("{}", max);
}
