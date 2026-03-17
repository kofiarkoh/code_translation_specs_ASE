fn sort_by_remainders(n: usize, m: usize, arr: &mut Vec<i32>) {
    // Preconditions: n and m must be positive integers
    if n == 0 || m == 0 || arr.len() != n {
        // Behavior is not defined for non-positive n or m, or incorrect array size
        return;
    }

    // Iterate over each divisor from 1 to m
    for divisor in 1..=m {
        // Sort the array based on the remainder when divided by the current divisor
        arr.sort_by(|a, b| {
            // Calculate remainders
            let remainder_a = a % divisor;
            let remainder_b = b % divisor;

            // Use the remainder for sorting; if equal, maintain stable sort
            remainder_a.cmp(&remainder_b).then_with(|| a.cmp(b))
        });
    }
}

fn main() {
    // Example usage
    let n = 5; // Number of integers
    let m = 3; // Maximum divisor
    let mut arr = vec![5, 3, 8, 6, 2]; // Array to be sorted

    sort_by_remainders(n, m, &mut arr);

    // Output the sorted array, each integer on a new line
    for num in arr {
        println!("{}", num);
    }
}
