fn calculate_transferable_resources(n: usize, a: &mut Vec<i64>, b: &Vec<i64>) -> i64 {
    // Ensure that the length of A is N + 1 and B is N
    assert_eq!(a.len(), n + 1);
    assert_eq!(b.len(), n);

    let mut total_transferable = 0;

    // Iterate through the elements of B
    for i in 0..n {
        // Calculate the transferable resources for each element
        // We use min to ensure we do not exceed the limits set by B
        let transferable = a[i] min b[i];
        total_transferable += transferable;

        // Update the available resources in A
        a[i] -= transferable;
    }

    // Return the total transferable resources
    total_transferable
}

fn main() {
    // Example usage
    let n = 3; // Number of elements in B
    let mut a = vec![10, 5, 3, 2]; // A has N + 1 elements
    let b = vec![4, 2, 1]; // B has N elements

    let total = calculate_transferable_resources(n, &mut a, &b);
    println!("{}", total); // Output the total transferable resources
}
