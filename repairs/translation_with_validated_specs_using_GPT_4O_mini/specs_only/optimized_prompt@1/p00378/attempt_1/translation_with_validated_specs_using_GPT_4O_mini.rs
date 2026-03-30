fn calculate_value(a: u32, b: u32, x: u32) -> u32 {
    // Preconditions: A, B, and X must be non-negative integers.
    // At least one of A or B must be greater than zero.
    assert!(a >= 0 && b >= 0 && x >= 0);
    assert!(a > 0 || b > 0);

    if a >= 2 * b {
        // If A is at least twice B, calculate using B.
        return ((x - 1) / 500 + 1) * b;
    } else {
        // If A is less than twice B, calculate based on X.
        let base_output = (x / 1000) * a;
        let remainder = x % 1000;

        if remainder > 0 {
            if remainder <= 500 && b < a {
                // If remainder is between 1 and 500 and B is less than A, add B.
                return base_output + b;
            } else {
                // Otherwise, add A.
                return base_output + a;
            }
        }
        // If remainder is 0, return the base output.
        return base_output;
    }
}
