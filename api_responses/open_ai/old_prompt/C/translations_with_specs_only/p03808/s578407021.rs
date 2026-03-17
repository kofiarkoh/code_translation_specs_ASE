fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements N
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Ensure N is within the valid range
    if n == 0 {
        println!("NO");
        return;
    }

    // Read the array A
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    // Calculate the sum of the elements in A
    let sum: i64 = a.iter().sum();

    // Calculate the triangular number T = N * (N + 1) / 2
    let triangular_number = (n as i64) * (n as i64 + 1) / 2;

    // Check if the sum is divisible by the triangular number
    if sum % triangular_number == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
