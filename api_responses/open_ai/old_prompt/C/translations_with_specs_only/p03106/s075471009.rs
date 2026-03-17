fn main() {
    // Read input values for a, b, and c
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Parse the integers a, b, and c
    let a: i32 = iter.next().unwrap().parse().expect("Invalid input for a");
    let b: i32 = iter.next().unwrap().parse().expect("Invalid input for b");
    let c: usize = iter.next().unwrap().parse().expect("Invalid input for c");

    // Ensure a, b, and c are positive integers
    if a <= 0 || b <= 0 || c == 0 {
        // If any of the inputs are invalid, we do not output anything
        return;
    }

    let mut common_divisors = Vec::new();

    // Find common divisors by iterating from 1 to min(a, b)
    let limit = a.min(b);
    for i in 1..=limit {
        if a % i == 0 && b % i == 0 {
            common_divisors.push(i);
        }
    }

    // Check if we have enough common divisors
    if common_divisors.len() < c {
        // If there are not enough common divisors, we do not output anything
        return;
    }

    // Output the c-th common divisor (1-based index)
    let result = common_divisors[common_divisors.len() - c];
    println!("{}", result);
}
