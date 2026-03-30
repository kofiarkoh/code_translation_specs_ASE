fn process_values(n: u32, mut a: i32, mut b: i32) -> (i32, i32) {
    // Calculate the effective number of iterations (N mod 12)
    let iterations = n % 12;

    // Perform the iterative updates
    for _ in 0..iterations {
        a -= b; // Update A by subtracting B
        b += a; // Update B by adding the new value of A
    }

    // Return the final values of A and B
    (a, b)
}
