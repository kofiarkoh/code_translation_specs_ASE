fn construct_number(n: i32, m: i32, s: &[i32], c: &[i32]) -> i32 {
    // Validate input constraints
    if n <= 0 || m < 0 || s.len() != m as usize || c.len() != m as usize {
        return -1; // Invalid input
    }

    // Initialize a vector to hold digit values, defaulting to -1 (unassigned)
    let mut digit = vec![-1; n as usize];

    // Process each digit-value pair
    for i in 0..m {
        let pos = s[i as usize];
        let val = c[i as usize];

        // Check if the position is valid
        if pos < 1 || pos > n || val < 0 || val > 9 {
            return -1; // Invalid digit position or value
        }

        // Check for conflicts in digit assignment
        let index = (pos - 1) as usize; // Convert to 0-based index
        if digit[index] != -1 && digit[index] != val {
            return -1; // Conflict detected
        }

        // Assign the digit value
        digit[index] = val;
    }

    // Construct the final number
    let mut num = 0;
    for &d in &digit {
        // Handle unassigned digits as zero
        num = num * 10 + if d == -1 { 0 } else { d };
    }

    num
}

fn main() {
    // Example usage
    let n = 5; // Number of digits
    let m = 3; // Number of digit-value pairs
    let s = [1, 3, 5]; // Digit positions
    let c = [2, 4, 6]; // Corresponding values

    let result = construct_number(n, m, &s, &c);
    println!("{}", result); // Output the constructed number
}
