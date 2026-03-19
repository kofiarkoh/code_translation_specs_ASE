use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 || q == 0 {
            break; // Exit if N or Q is zero, as per the specification.
        }

        let mut p = vec![0; n + 1];
        let mut m = vec![0; n + 1];
        let mut r: i64 = 0;

        // Initialize P and M
        for i in 0..n {
            p[i + 1] = i + 1; // P[i] = i + 1
            m[i + 1] = 0; // M[i] = 0
        }
        m[1] = 1; // M[1] = 1

        // Read the mapping
        for i in 2..=n {
            let t: usize = iter.next().unwrap().parse().unwrap();
            p[i] = t; // P[i] = t
        }

        let mut queries = Vec::with_capacity(q);
        let mut values = Vec::with_capacity(q);

        // Read the queries
        for _ in 0..q {
            let mut query_line = String::new();
            let _ = reader.read_line(&mut query_line).unwrap(); // Read the line
            let mut query_parts = query_line.split_whitespace();
            let query_char: char = query_parts.next().unwrap().chars().next().unwrap();
            let value: usize = query_parts.next().unwrap().parse().unwrap();
            queries.push(query_char);
            values.push(value);
            if query_char == 'M' {
                m[value] += 1; // Increment modification count
            }
        }

        // Process the queries in reverse order
        for i in (0..q).rev() {
            if queries[i] == 'M' {
                m[values[i]] -= 1; // Decrement modification count
            } else {
                let mut t = values[i];
                while m[t] == 0 {
                    t = p[t]; // Follow the mapping
                }
                r += t as i64; // Accumulate the result
            }
        }

        println!("{}", r); // Output the cumulative result
    }
}
