fn sort_by_remainders(n: usize, m: usize, arr: &mut Vec<i32>) {
    // Preconditions: n and m must be positive integers
    // The behavior is not defined if n or m are non-positive or if fewer than n integers are provided.
    if n == 0 || m == 0 || arr.len() != n {
        return; // Undefined behavior as per specification
    }

    // Iterate over each divisor from 1 to m
    for divisor in 1..=m {
        // Sort the array based on the remainder when divided by the current divisor
        arr.sort_by_key(|&x| ((x % divisor as i32 + divisor as i32) % divisor as i32)); // Ensures non-negative remainders
    }
}

fn main() {
    let n = 5; // Example number of integers
    let m = 3; // Example maximum divisor
    let mut arr = vec![5, 3, 8, 1, 2]; // Example array to be sorted

    sort_by_remainders(n, m, &mut arr);

    // Output the sorted array, each integer on a new line
    for num in arr {
        println!("{}", num);
    }
}
