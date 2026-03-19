use std::io::{self, BufRead};

fn enumeration(result: &mut Vec<i32>, used: &mut Vec<i32>, n: usize, d: usize) {
    if d >= n {
        // Print the current permutation
        let output = result.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", output);
        return;
    }

    let mut used2 = Vec::with_capacity(n - d - 1);
    for i in 0..(n - d) {
        result[d] = used[i];
        // Create a new used array excluding the current index
        used2.clear();
        for j in 0..(n - d) {
            if j != i {
                used2.push(used[j]);
            }
        }
        enumeration(result, &mut used2, n, d + 1);
    }
}

fn main() {
    let stdin = io::stdin();
    let n: usize = stdin.lock().lines()
        .next()
        .expect("Failed to read line")
        .expect("Failed to parse input")
        .trim()
        .parse()
        .expect("Input must be a non-negative integer");

    let mut used: Vec<i32> = (1..=n as i32).collect();
    let mut result: Vec<i32> = vec![0; n];

    enumeration(&mut result, &mut used, n, 0);
}
