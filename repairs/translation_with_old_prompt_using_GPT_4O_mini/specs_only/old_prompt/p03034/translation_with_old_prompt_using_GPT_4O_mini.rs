use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => {
            println!("0");
            return;
        }
    };

    // Ensure n is greater than 2
    if n <= 2 {
        println!("0");
        return;
    }

    // Read the array of long long integers
    let mut a: Vec<i64> = Vec::with_capacity(n);
    let line = lines.next().unwrap().unwrap();
    a.extend(line.split_whitespace().filter_map(|s| s.parse::<i64>().ok()));

    // Ensure we have exactly n elements
    if a.len() != n {
        println!("0");
        return;
    }

    let mut ans = 0;

    // Iterate through pairs of elements with the specified distance
    for i in 0..n {
        for j in (i + 2)..n { // Ensure elements are not adjacent
            ans = ans.max(a[i] + a[j]); // Update maximum sum if a valid pair is found
        }
    }

    // Print the result
    println!("{}", ans);
}
