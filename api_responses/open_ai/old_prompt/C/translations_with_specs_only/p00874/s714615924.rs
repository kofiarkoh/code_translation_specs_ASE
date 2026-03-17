fn main() {
    // Arrays to hold counts of item types for both groups, initialized to zero.
    let mut a = [0; 21]; // For item types 1 to 20
    let mut b = [0; 21]; // For item types 1 to 20

    // Read the first input which indicates the number of items in the first group.
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let w: usize = input.trim().parse().expect("Invalid input for w");

    // If w is zero, terminate without processing further input.
    if w == 0 {
        return;
    }

    // Read the item types for the first group.
    for _ in 0..w {
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let item_type: usize = input.trim().parse().expect("Invalid item type");
        if item_type >= 1 && item_type <= 20 {
            a[item_type] += 1; // Increment count for the item type in group a
        }
    }

    // Read the number of items in the second group.
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let d: usize = input.trim().parse().expect("Invalid input for d");

    // Read the item types for the second group.
    for _ in 0..d {
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let item_type: usize = input.trim().parse().expect("Invalid item type");
        if item_type >= 1 && item_type <= 20 {
            b[item_type] += 1; // Increment count for the item type in group b
        }
    }

    // Calculate the weighted sum based on the maximum counts from both groups.
    let mut weighted_sum = 0;
    for i in 1..=20 {
        weighted_sum += a[i].max(b[i]); // Use the maximum count of each item type
    }

    // Output the calculated weighted sum followed by a newline.
    println!("{}", weighted_sum);
}
