use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the length of the string
    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().expect("Invalid length");

    // Read the string
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let s: Vec<char> = s.trim().chars().collect();

    // Read the number of queries
    let mut q = String::new();
    reader.read_line(&mut q).unwrap();
    let q: usize = q.trim().parse().expect("Invalid number of queries");

    // Read the query lengths
    let mut k = Vec::new();
    for _ in 0..q {
        let mut query_length = String::new();
        reader.read_line(&mut query_length).unwrap();
        let length: usize = query_length.trim().parse().expect("Invalid query length");
        k.push(length);
    }

    // Process each query
    for &query_length in &k {
        let mut ans: i64 = 0;
        let mut d: i64 = 0;
        let mut m: i64 = 0;
        let mut dm: i64 = 0;

        // Initial window
        for i in 0..query_length {
            match s[i] {
                'D' => d += 1,
                'M' => {
                    m += 1;
                    dm += d;
                }
                'C' => ans += dm,
                _ => unreachable!(), // According to the spec, only 'D', 'M', 'C' are valid
            }
        }

        // Sliding window
        for i in query_length..n {
            // Remove the character going out of the window
            match s[i - query_length] {
                'D' => {
                    dm -= m;
                    d -= 1;
                }
                'M' => m -= 1,
                'C' => {}
                _ => unreachable!(),
            }

            // Add the new character coming into the window
            match s[i] {
                'D' => d += 1,
                'M' => {
                    m += 1;
                    dm += d;
                }
                'C' => ans += dm,
                _ => unreachable!(),
            }
        }

        // Output the result for the current query
        println!("{}", ans);
    }
}
