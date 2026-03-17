use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut inputs = input.split_whitespace();

    // Read the two long integers from input
    let a: i64 = inputs.next().unwrap().parse().unwrap();
    let b: i64 = inputs.next().unwrap().parse().unwrap();

    // Preconditions check
    if a == 0 || b <= 0 {
        return; // Behavior is not defined for invalid inputs
    }

    // Find the smallest multiple of `a` that is also a multiple of `b`
    for i in 1..=b + 1 {
        let multiple = a * i;
        if multiple % b == 0 {
            println!("{}", multiple);
            break; // Output the first valid multiple and exit
        }
    }
}
