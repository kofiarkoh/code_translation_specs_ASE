fn main() {
    // Read input values for L, R, and d
    let (l, r, d) = read_input();

    // Calculate the count of integers divisible by d in the range [L, R]
    let count = count_divisibles(l, r, d);

    // Output the result
    println!("{}", count);
}

// Function to read input values
fn read_input() -> (i32, i32, i32) {
    // Assuming input is provided in a valid format as per the specification
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Unwrap the values into L, R, and d
    (nums[0], nums[1], nums[2])
}

// Function to count integers divisible by d in the range [L, R]
fn count_divisibles(l: i32, r: i32, d: i32) -> i32 {
    // Preconditions guarantee that d > 0, so we can safely perform the calculations
    let count_r = r / d; // Count of multiples of d up to R
    let count_l = (l - 1) / d; // Count of multiples of d up to L-1

    // The count of multiples of d in the range [L, R] is the difference
    count_r - count_l
}
