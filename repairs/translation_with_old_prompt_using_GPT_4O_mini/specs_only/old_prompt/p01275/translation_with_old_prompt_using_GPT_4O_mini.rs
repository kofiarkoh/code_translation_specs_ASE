fn min_rotations(n: usize, init: &str, unlock: &str) -> usize {
    // Initialize the minimum rotation count to a large value
    let mut min_rotations = usize::MAX;

    // Function to calculate the rotation count between two digits
    fn rotation_count(from: char, to: char) -> usize {
        let from_digit = from.to_digit(10).unwrap();
        let to_digit = to.to_digit(10).unwrap();
        let direct = (to_digit as isize - from_digit as isize).abs() as usize;
        let wrap_around = 10 - direct;
        direct.min(wrap_around)
    }

    // Check all possible rotations of the `init` string
    for i in 0..n {
        let mut total_rotations = 0;
        for j in 0..n {
            total_rotations += rotation_count(init.chars().nth((i + j) % n).unwrap(), unlock.chars().nth(j).unwrap());
        }
        min_rotations = min_rotations.min(total_rotations);
    }

    // If the strings are identical, return 0
    if min_rotations == usize::MAX {
        return 0;
    }

    // Return the minimum rotations found
    min_rotations
}

fn main() {
    // Read input values
    let n = 4; // Length of the digit strings
    let init = "1234"; // Initial string
    let unlock = "5678"; // Target string
    let result = min_rotations(n, init, unlock);
    println!("{}", result); // Output the minimum rotations
}
