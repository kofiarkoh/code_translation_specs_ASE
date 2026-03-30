fn compute_value(t: [i64; 3], a: [i64; 3], b: [i64; 3]) -> String {
    // Ensure T[1] and T[2] are not zero to avoid division by zero
    if t[1] == 0 || t[2] == 0 {
        panic!("T[1] and T[2] must not be zero.");
    }

    // Check if A[1] is greater than B[1] to determine if modifications are needed
    let mut a1 = a[1];
    let mut b1 = b[1];

    if a1 > b1 {
        // Modify A and B based on the specification
        a1 -= b1; // Example modification, adjust as per actual logic
        b1 = 0;   // Example modification, adjust as per actual logic
    }

    // Calculate the linear combination
    if a1 * t[1] == b1 * t[2] {
        return "infinity".to_string();
    } else {
        // Calculate the derived result based on the differences
        let difference = a1 - b1;
        return difference.to_string();
    }
}
