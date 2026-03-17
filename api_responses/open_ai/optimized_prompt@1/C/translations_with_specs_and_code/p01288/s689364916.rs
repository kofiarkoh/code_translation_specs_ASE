use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace();
        let n: usize = nums.next().unwrap().parse().unwrap();
        let q: usize = nums.next().unwrap().parse().unwrap();

        let mut p = vec![0; n + 1];
        let mut m = vec![0; n + 1];
        let mut r: i64 = 0;

        // Initialize the mapping and modification counts
        for i in 0..n {
            p[i + 1] = i + 1; // 1-based indexing
            m[i + 1] = 0; // Initialize modification counts to 0
        }
        m[1] = 1; // Set the first modification count

        // Read the mapping values
        for i in 2..=n {
            let line = lines.next().unwrap().unwrap();
            let t: usize = line.trim().parse().unwrap();
            p[i] = t;
        }

        let mut queries = Vec::with_capacity(q);
        for _ in 0..q {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            let query_type = parts[0].chars().next().unwrap();
            let value: usize = parts[1].parse().unwrap();
            queries.push((query_type, value));
            if query_type == 'M' {
                m[value] += 1; // Increment modification count
            }
        }

        // Process queries in reverse order
        for (query_type, value) in queries.iter().rev() {
            if *query_type == 'M' {
                m[*value] -= 1; // Decrement modification count
            } else {
                let mut t = *value;
                while m[t] == 0 {
                    t = p[t]; // Follow the mapping until a valid modification is found
                }
                r += t as i64; // Accumulate the result
            }
        }

        println!("{}", r); // Output the cumulative result
    }
}
