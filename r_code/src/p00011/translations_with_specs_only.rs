fn main() {
    // Example inputs
    let width: usize = 5; // Width of the list (must be positive)
    let num_swaps: usize = 2; // Number of swap operations (must be non-negative)
    let swaps: Vec<(usize, usize)> = vec![(1, 3), (2, 5)]; // Pairs of indices to swap

    // Preconditions check
    if width == 0 || num_swaps < 0 {
        // Undefined behavior for invalid width or number of swaps
        return;
    }

    // Initialize the list from 1 to width
    let mut list: Vec<usize> = (1..=width).collect();

    // Perform the swaps
    for (i, j) in swaps {
        // Check if indices are within bounds (1 to width)
        if i < 1 || i > width || j < 1 || j > width {
            // Undefined behavior for out-of-bounds indices
            return;
        }
        // Swap the elements at the specified indices (convert to 0-based)
        list.swap(i - 1, j - 1);
    }

    // Output the final state of the list
    for number in list {
        println!("{}", number);
    }
}
