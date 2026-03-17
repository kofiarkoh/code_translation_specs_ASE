use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut inputs = input.split_whitespace();
    let a: i32 = inputs.next().unwrap().parse().expect("Invalid input");
    let b: i32 = inputs.next().unwrap().parse().expect("Invalid input");
    let c: usize = inputs.next().unwrap().parse().expect("Invalid input");

    // Preconditions: a and b must be non-negative, c must be positive
    if a < 0 || b < 0 || c == 0 {
        // According to the spec, we do not handle invalid input formats or negative integers.
        // The behavior is undefined in such cases.
        return;
    }

    let mut sum = 0;
    let mut l = -1; // Using -1 to indicate undefined value

    let max_divisor = a.max(b);
    for i in (1..=max_divisor).rev() {
        if a % i == 0 && b % i == 0 {
            sum += 1;
            if sum == c {
                l = i;
                break; // Found the c-th common divisor
            }
        }
    }

    // Output the result
    if l != -1 {
        println!("{}", l);
    } else {
        // If the position exceeds the number of common divisors, output is undefined.
        // The spec does not specify what to do in this case, so we simply do nothing.
    }
}
