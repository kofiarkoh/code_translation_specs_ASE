use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a non-negative integer");

    // Preconditions check: n must be less than or equal to 99
    if n > 99 {
        // Behavior is not defined for n > 99, but we will not handle it explicitly as per the spec
        return;
    }

    let mut l = [0; 100];
    l[0] = 2; // First term
    l[1] = 1; // Second term

    // Compute the nth term based on the recurrence relation
    for i in 2..=n {
        l[i] = l[i - 2] + l[i - 1];
    }

    // Output the nth term
    println!("{}", l[n]);
}
