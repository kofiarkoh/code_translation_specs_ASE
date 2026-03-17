use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of strings in the first set
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read the first set of strings
    let mut first_set = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let s = input.trim().to_string();
        first_set.push(s);
    }

    // Read the number of strings in the second set
    input.clear();
    reader.read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().unwrap();

    // Read the second set of strings
    let mut second_set = Vec::with_capacity(m);
    for _ in 0..m {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let t = input.trim().to_string();
        second_set.push(t);
    }

    // Calculate the maximum adjusted frequency
    let mut max_frequency = 0;

    for i in 0..n {
        let mut frequency = 0;

        // Count occurrences of the string in the first set
        for j in 0..n {
            if first_set[i] == first_set[j] {
                frequency += 1;
            }
        }

        // Adjust frequency based on occurrences in the second set
        for j in 0..m {
            if first_set[i] == second_set[j] {
                frequency -= 1;
            }
        }

        // Update max frequency if necessary
        if frequency > max_frequency {
            max_frequency = frequency;
        }
    }

    // Output the result
    println!("{}", max_frequency);
}
