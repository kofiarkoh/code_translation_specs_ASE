use std::io::{self, Write};

fn main() {
    let mut T = [0; 3];
    let mut A = [0; 3];
    let mut B = [0; 3];

    // Read input values
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let values: Vec<&str> = input.split_whitespace().collect();
    T[1] = values[0].parse::<i64>().unwrap();
    T[2] = values[1].parse::<i64>().unwrap();

    input.clear();
    stdin.read_line(&mut input).unwrap();
    let values: Vec<&str> = input.split_whitespace().collect();
    A[1] = values[0].parse::<i64>().unwrap();
    A[2] = values[1].parse::<i64>().unwrap();

    input.clear();
    stdin.read_line(&mut input).unwrap();
    let values: Vec<&str> = input.split_whitespace().collect();
    B[1] = values[0].parse::<i64>().unwrap();
    B[2] = values[1].parse::<i64>().unwrap();

    // Check for infinite solutions
    if A[1] * T[1] + A[2] * T[2] == B[1] * T[1] + B[2] * T[2] {
        println!("infinity");
        io::stdout().flush().unwrap();
        return;
    }

    // Swap A and B if A[1] is greater than B[1]
    if A[1] > B[1] {
        A[1] += B[1];
        B[1] = A[1] - B[1];
        A[1] -= B[1];
        A[2] += B[2];
        B[2] = A[2] - B[2];
        A[2] -= B[2];
    }

    // Calculate the result based on the conditions
    let delta_a = B[1] - A[1];
    let delta_b = A[2] - B[2];
    let t1 = T[1];
    let t2 = T[2];

    if delta_a * t1 > delta_b * t2 {
        println!("0");
    } else {
        let denominator = delta_b * t2 - delta_a * t1;
        if delta_a * t1 % denominator == 0 {
            println!("{}", (delta_a * t1 / denominator) * 2);
        } else {
            println!("{}", (delta_a * t1 / denominator) * 2 + 1);
        }
    }
    io::stdout().flush().unwrap();
}
