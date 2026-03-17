fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements in the list
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut numbers: Vec<i64> = Vec::with_capacity(n);

    // Read the list of integers
    for _ in 0..n {
        let number: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        numbers.push(number);
    }

    // Compute the absolute sum
    let absolute_sum: i64 = numbers.iter().map(|&x| x.abs()).sum();
    println!("{}", absolute_sum);

    // Determine the state of the list
    let all_non_negative = numbers.iter().all(|&x| x >= 0);
    let all_non_positive = numbers.iter().all(|&x| x <= 0);

    // Output pairs based on the state of the list
    if all_non_negative {
        // Case: All non-negative
        for &num in &numbers {
            println!("{} {}", num, num);
        }
    } else if all_non_positive {
        // Case: All non-positive
        for &num in &numbers {
            println!("{} {}", num, num);
        }
    } else {
        // Case: Mixed
        for &num in &numbers {
            println!("{} {}", num, num.abs());
        }
    }
}
