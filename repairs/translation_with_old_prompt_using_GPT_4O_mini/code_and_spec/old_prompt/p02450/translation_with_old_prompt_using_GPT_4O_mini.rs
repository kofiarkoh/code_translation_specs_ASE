use std::io::{self, Write};

fn enumeration(result: &mut Vec<i32>, used: &mut Vec<i32>, n: usize, d: usize) {
    if d >= n {
        // Print the current permutation
        let output: Vec<String> = result.iter().map(|&x| x.to_string()).collect();
        println!("{}", output.join(" "));
        return;
    }

    let mut used2 = Vec::with_capacity(n - d - 1);
    for i in 0..(n - d) {
        result[d] = used[i];
        // Fill used2 with elements from used excluding the current one
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid integer");

    let mut used: Vec<i32> = (1..=n as i32).collect();
    let mut result: Vec<i32> = vec![0; n];

    enumeration(&mut result, &mut used, n, 0);
}
