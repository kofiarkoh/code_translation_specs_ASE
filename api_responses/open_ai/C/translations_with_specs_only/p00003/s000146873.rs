fn main() {
    use std::io::{self, BufRead};

    // Read the number of sets of integers
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line to get the number of sets
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Process each set of three integers
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Ensure we have exactly three integers
        if nums.len() != 3 {
            continue; // This should not happen according to the specification
        }

        // Check all permutations of the integers to see if they satisfy the Pythagorean theorem
        let (a, b, c) = (nums[0], nums[1], nums[2]);
        if is_right_triangle(a, b, c) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

// Function to check if the given sides can form a right triangle
fn is_right_triangle(a: i32, b: i32, c: i32) -> bool {
    // Check the Pythagorean theorem for all permutations
    a * a + b * b == c * c || a * a + c * c == b * b || b * b + c * c == a * a
}
