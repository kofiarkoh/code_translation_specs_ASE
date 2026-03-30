fn compute_value(n: u32, k: u32) -> u32 {
    // Preconditions: K must not be zero to avoid division by zero.
    // If K is zero, the behavior is undefined, so we do not handle it explicitly.
    
    if k == 0 {
        panic!("K must not be zero."); // This will cause a panic if K is zero.
    }

    let quotient = n / k; // Integer division of N by K
    let cube_quotient = quotient.pow(3); // Cube of the quotient

    if k % 2 == 1 { // K is odd
        cube_quotient // Return the cube of the quotient
    } else { // K is even
        let modified_quotient = (n + k / 2) / k; // Integer division of (N + K/2) by K
        let cube_modified_quotient = modified_quotient.pow(3); // Cube of the modified quotient
        cube_quotient + cube_modified_quotient // Return the sum of the cubes
    }
}

fn main() {
    let n = 10; // Example input for N
    let k = 2;  // Example input for K
    let result = compute_value(n, k);
    println!("{}", result); // Output the computed value
}
