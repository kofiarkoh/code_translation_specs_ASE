use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a positive integer");

    // Preconditions check
    if n == 0 {
        println!("NO");
        return;
    }

    let mut a = vec![0; n + 1];
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Input must be integers"))
        .collect();

    // Ensure we have exactly N integers
    if numbers.len() != n {
        println!("NO");
        return;
    }

    for i in 0..n {
        a[i] = numbers[i];
    }
    a[n] = a[0]; // Mimicking the C code behavior

    let mut sum = 0;
    let mut diff = vec![0; n + 1];

    for i in 1..=n {
        sum += a[i];
        diff[i] = a[i] - a[i - 1];
    }

    // Check divisibility condition
    if sum % (n * (n + 1) / 2) != 0 {
        println!("NO");
    } else {
        sum /= n * (n + 1) / 2;
        diff[0] = 0;
        let mut valid = true;

        for i in 1..=n {
            diff[i] = (sum - diff[i]) / n;
            if diff[i] < 0 || diff[i] > sum {
                valid = false;
                break;
            }
            diff[0] += diff[i];
        }

        if !valid || diff[0] != sum {
            println!("NO");
        } else {
            for i in 1..=n {
                a[n] -= diff[i] * (n - i + 1);
            }
            if a[n] != 0 {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
    io::stdout().flush().expect("Failed to flush stdout");
}
