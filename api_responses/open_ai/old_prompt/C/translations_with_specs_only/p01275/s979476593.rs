fn min_rotations(n: usize, init: &str, unlock: &str) -> usize {
    // Initialize the minimum rotation count to a large value
    let mut min_rotations = usize::MAX;

    // Function to calculate the number of rotations needed to match the strings
    fn rotate_count(init: &str, unlock: &str) -> usize {
        let mut count = 0;
        for (i, c) in init.chars().enumerate() {
            // Calculate the difference considering circular rotation
            let diff = (unlock.chars().nth(i).unwrap() as i32 - c as i32 + 10) % 10;
            count += diff as usize;
        }
        count
    }

    // Check for the case where the strings are identical
    if init == unlock {
        return 0;
    }

    // Generate all rotations of the initial string
    let mut current = init.to_string();
    for _ in 0..n {
        // Calculate the rotations needed for the current configuration
        let rotations = rotate_count(&current, unlock);
        min_rotations = min_rotations.min(rotations);

        // Rotate the string by moving the first character to the end
        current.push(current.remove(0));
    }

    min_rotations
}

fn main() {
    // Example input
    let n = 3; // Length of the digit strings
    let init = "123"; // Initial state
    let unlock = "231"; // Target state

    // Calculate and print the minimum number of rotations
    let result = min_rotations(n, init, unlock);
    println!("{}", result);
}
