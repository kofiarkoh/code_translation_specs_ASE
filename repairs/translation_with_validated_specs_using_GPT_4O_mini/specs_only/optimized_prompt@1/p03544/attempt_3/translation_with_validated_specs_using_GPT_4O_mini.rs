fn compute_nth_term(n: usize) -> i64 {
    // The sequence is defined for n in the range [0, 99].
    // We will use an array to store the terms of the sequence.
    let mut sequence = [0i64; 100];

    // Base cases
    sequence[0] = 2; // l[0] = 2
    sequence[1] = 1; // l[1] = 1

    // Compute terms for n >= 2
    for i in 2..=n {
        sequence[i] = sequence[i - 2] + sequence[i - 1]; // l[n] = l[n-2] + l[n-1]
    }

    // Return the nth term
    sequence[n]
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid number");
    
    let result = compute_nth_term(n);
    println!("{}", result); // Output only the result as expected
}
